use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::{Command, Stdio};
use tracing::{debug, info};

/// Instance model representing a Google Cloud VM instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    /// Instance ID
    pub id: String,
    /// Instance name
    pub name: String,
    /// Instance status
    pub status: String,
    /// Machine type (e.g., e2-micro, n1-standard-1)
    pub machine_type: String,
    /// Zone where the instance is located
    pub zone: String,
    /// External IP address, if any
    pub external_ip: Option<String>,
    /// Internal IP address
    pub internal_ip: Option<String>,
    /// Creation timestamp
    pub creation_timestamp: Option<String>,
    /// Description
    pub description: Option<String>,
    /// Metadata as key-value pairs
    pub metadata: Option<HashMap<String, String>>,
    /// Tags
    pub tags: Vec<String>,
}

/// Simplified model for instance data coming from gcloud CLI
#[derive(Debug, Clone, Deserialize)]
struct GcloudInstance {
    id: String,
    name: String,
    status: String,
    #[serde(rename = "machineType")]
    machine_type: String,
    zone: String,
    #[serde(rename = "networkInterfaces")]
    network_interfaces: Option<Vec<NetworkInterface>>,
    #[serde(rename = "creationTimestamp")]
    creation_timestamp: Option<String>,
    description: Option<String>,
    metadata: Option<InstanceMetadata>,
    tags: Option<Tags>,
}

#[derive(Debug, Clone, Deserialize)]
struct NetworkInterface {
    #[serde(rename = "networkIP")]
    network_ip: Option<String>,
    #[serde(rename = "accessConfigs")]
    access_configs: Option<Vec<AccessConfig>>,
}

#[derive(Debug, Clone, Deserialize)]
struct AccessConfig {
    #[serde(rename = "natIP")]
    nat_ip: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct InstanceMetadata {
    items: Option<Vec<MetadataItem>>,
}

#[derive(Debug, Clone, Deserialize)]
struct MetadataItem {
    key: Option<String>,
    value: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct Tags {
    items: Option<Vec<String>>,
}

impl From<GcloudInstance> for Instance {
    fn from(gcloud_instance: GcloudInstance) -> Self {
        let mut external_ip = None;
        let mut internal_ip = None;
        
        // Extract IP addresses from network interfaces
        if let Some(network_interfaces) = gcloud_instance.network_interfaces {
            for iface in network_interfaces {
                // Internal IP
                if let Some(ip) = iface.network_ip {
                    internal_ip = Some(ip);
                }
                
                // External IP
                if let Some(access_configs) = iface.access_configs {
                    for config in access_configs {
                        if let Some(nat_ip) = config.nat_ip {
                            external_ip = Some(nat_ip);
                            break;
                        }
                    }
                }
            }
        }
        
        // Extract metadata
        let metadata = gcloud_instance.metadata.and_then(|meta| {
            meta.items.map(|items| {
                items.into_iter()
                     .filter_map(|item| {
                        match (item.key, item.value) {
                            (Some(key), Some(value)) => Some((key, value)),
                            _ => None,
                        }
                    })
                    .collect::<HashMap<String, String>>()
            })
        });
        
        // Extract tags
        let tags = gcloud_instance.tags
            .and_then(|tags| tags.items)
            .unwrap_or_default();
        
        // Extract zone from zone URL
        let zone = gcloud_instance.zone
            .split('/')
            .last()
            .unwrap_or("unknown")
            .to_string();
        
        // Extract machine type from machine type URL
        let machine_type = gcloud_instance.machine_type
            .split('/')
            .last()
            .unwrap_or("unknown")
            .to_string();
        
        Self {
            id: gcloud_instance.id,
            name: gcloud_instance.name,
            status: gcloud_instance.status,
            machine_type,
            zone,
            external_ip,
            internal_ip,
            creation_timestamp: gcloud_instance.creation_timestamp,
            description: gcloud_instance.description,
            metadata,
            tags,
        }
    }
}

/// List all instances in a project
pub async fn list_instances(project_id: &str, json_output: bool) -> Result<Vec<Instance>> {
    info!("Listing instances for project: {}", project_id);
    
    // Build command
    let mut cmd = Command::new("gcloud");
    cmd.args([
        "compute", 
        "instances", 
        "list", 
        "--project", project_id,
    ]);
    
    // Add format flags
    if json_output {
        cmd.args(["--format", "json"]);
    }
    
    // Execute command
    let output = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .context("Failed to execute gcloud compute instances list command")?;
    
    // Check if command was successful
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Failed to list instances: {}", error));
    }
    
    // Parse JSON output
    let stdout = String::from_utf8_lossy(&output.stdout);
    let gcloud_instances: Vec<GcloudInstance> = serde_json::from_str(&stdout)
        .context("Failed to parse instance list JSON")?;
    
    // Convert to our model
    let instances: Vec<Instance> = gcloud_instances.into_iter()
        .map(Instance::from)
        .collect();
    
    debug!("Found {} instances", instances.len());
    
    Ok(instances)
}

/// Get a specific instance by name or ID
pub async fn get_instance(project_id: &str, instance_id: &str, json_output: bool) -> Result<Instance> {
    info!("Getting instance {} in project {}", instance_id, project_id);
    
    // First we need to find which zone the instance is in
    let instances = list_instances(project_id, json_output).await?;
    
    // Find the instance by ID or name
    let instance = instances.into_iter()
        .find(|i| i.id == instance_id || i.name == instance_id)
        .ok_or_else(|| anyhow::anyhow!("Instance not found: {}", instance_id))?;
    
    // Now get detailed information
    let mut cmd = Command::new("gcloud");
    cmd.args([
        "compute", 
        "instances", 
        "describe", 
        &instance.name,
        "--zone", &instance.zone,
        "--project", project_id,
    ]);
    
    // Add format flags
    if json_output {
        cmd.args(["--format", "json"]);
    }
    
    // Execute command
    let output = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .context("Failed to execute gcloud compute instances describe command")?;
    
    // Check if command was successful
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Failed to get instance details: {}", error));
    }
    
    // Parse JSON output
    let stdout = String::from_utf8_lossy(&output.stdout);
    let gcloud_instance: GcloudInstance = serde_json::from_str(&stdout)
        .context("Failed to parse instance details JSON")?;
    
    Ok(Instance::from(gcloud_instance))
}

/// Start an instance
pub async fn start_instance(project_id: &str, instance_id: &str) -> Result<()> {
    info!("Starting instance {} in project {}", instance_id, project_id);
    
    // First we need to find which zone the instance is in
    let instance = get_instance(project_id, instance_id, true).await?;
    
    // Build command
    let mut cmd = Command::new("gcloud");
    cmd.args([
        "compute", 
        "instances", 
        "start", 
        &instance.name,
        "--zone", &instance.zone,
        "--project", project_id,
        "--quiet", // Disable interactive prompts
    ]);
    
    // Execute command
    let output = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .context("Failed to execute gcloud compute instances start command")?;
    
    // Check if command was successful
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Failed to start instance: {}", error));
    }
    
    info!("Successfully started instance {}", instance.name);
    Ok(())
}

/// Stop an instance
pub async fn stop_instance(project_id: &str, instance_id: &str) -> Result<()> {
    info!("Stopping instance {} in project {}", instance_id, project_id);
    
    // First we need to find which zone the instance is in
    let instance = get_instance(project_id, instance_id, true).await?;
    
    // Build command
    let mut cmd = Command::new("gcloud");
    cmd.args([
        "compute", 
        "instances", 
        "stop", 
        &instance.name,
        "--zone", &instance.zone,
        "--project", project_id,
        "--quiet", // Disable interactive prompts
    ]);
    
    // Execute command
    let output = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .context("Failed to execute gcloud compute instances stop command")?;
    
    // Check if command was successful
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Failed to stop instance: {}", error));
    }
    
    info!("Successfully stopped instance {}", instance.name);
    Ok(())
}

/// Restart an instance (stop then start)
pub async fn restart_instance(project_id: &str, instance_id: &str) -> Result<()> {
    info!("Restarting instance {} in project {}", instance_id, project_id);
    
    // First we need to find which zone the instance is in
    let instance = get_instance(project_id, instance_id, true).await?;
    
    // Build command
    let mut cmd = Command::new("gcloud");
    cmd.args([
        "compute", 
        "instances", 
        "reset", // reset is like a power cycle/restart
        &instance.name,
        "--zone", &instance.zone,
        "--project", project_id,
        "--quiet", // Disable interactive prompts
    ]);
    
    // Execute command
    let output = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .context("Failed to execute gcloud compute instances reset command")?;
    
    // Check if command was successful
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Failed to restart instance: {}", error));
    }
    
    info!("Successfully restarted instance {}", instance.name);
    Ok(())
}

/// Delete an instance
pub async fn delete_instance(project_id: &str, instance_id: &str) -> Result<()> {
    info!("Deleting instance {} in project {}", instance_id, project_id);
    
    // First we need to find which zone the instance is in
    let instance = get_instance(project_id, instance_id, true).await?;
    
    // Build command
    let mut cmd = Command::new("gcloud");
    cmd.args([
        "compute", 
        "instances", 
        "delete", 
        &instance.name,
        "--zone", &instance.zone,
        "--project", project_id,
        "--quiet", // Disable interactive prompts
    ]);
    
    // Execute command
    let output = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .context("Failed to execute gcloud compute instances delete command")?;
    
    // Check if command was successful
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Failed to delete instance: {}", error));
    }
    
    info!("Successfully deleted instance {}", instance.name);
    Ok(())
}