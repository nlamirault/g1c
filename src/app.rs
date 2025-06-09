use anyhow::{Context, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{backend::Backend, Terminal};
use std::time::{Duration, Instant};
use tracing::{debug, error, info};

use crate::cloud::CloudClient;
use crate::config::Config;
use crate::ui::{self, Action, UiState};

/// Main application state
pub struct App {
    /// Application configuration
    config: Config,
    /// Cloud API client
    cloud_client: CloudClient,
    /// UI state
    ui_state: UiState,
    /// Whether the app should exit
    should_quit: bool,
    /// Last refresh time
    last_refresh: Instant,
}

impl App {
    /// Create a new application instance
    pub async fn new(config: Config) -> Result<Self> {
        // Initialize cloud client
        let cloud_client = CloudClient::new(&config)
            .await
            .context("Failed to initialize cloud client")?;

        // Create initial UI state
        let ui_state = UiState::new();

        // Initialize UI state with cloud client info
        let mut app = Self {
            config,
            cloud_client,
            ui_state,
            should_quit: false,
            last_refresh: Instant::now(),
        };

        // Update UI state with cloud client info
        app.update_ui_info();

        // Initial data fetch
        app.refresh_data().await?;

        Ok(app)
    }

    /// Run the application main loop
    pub async fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        // Main event loop
        while !self.should_quit {
            // Draw UI
            terminal.draw(|frame| ui::render(frame, &self.ui_state))?;

            // Handle events
            self.handle_events().await?;

            // Check if we need to refresh data
            if self.last_refresh.elapsed() >= Duration::from_secs(self.config.refresh_interval) {
                self.refresh_data().await?;
            }
        }

        Ok(())
    }

    /// Handle terminal events
    async fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                self.handle_key_event(key).await?;
            }
        }

        Ok(())
    }

    /// Handle a key event
    async fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        debug!("Key event: {:?}", key);

        match key.code {
            // Quit
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.should_quit = true
            }

            // Help
            KeyCode::Char('?') => self.ui_state.toggle_help(),

            // Navigation
            KeyCode::Up | KeyCode::Char('k') => self.ui_state.previous_item(),
            KeyCode::Down | KeyCode::Char('j') => self.ui_state.next_item(),
            KeyCode::Enter => self.ui_state.show_details(),
            KeyCode::Esc => self.ui_state.close_popup(),

            // Refresh
            KeyCode::Char('r') => {
                self.refresh_data().await?;
            }

            // Instance actions
            KeyCode::Char('s') => {
                if let Some(instance_id) = self.ui_state.selected_instance_id() {
                    self.perform_action(Action::Start, instance_id).await?;
                }
            }
            KeyCode::Char('S') => {
                if let Some(instance_id) = self.ui_state.selected_instance_id() {
                    self.perform_action(Action::Stop, instance_id).await?;
                }
            }
            KeyCode::Char('R') => {
                if let Some(instance_id) = self.ui_state.selected_instance_id() {
                    self.perform_action(Action::Restart, instance_id).await?;
                }
            }

            // Filter
            KeyCode::Char('f') => self.ui_state.toggle_filter_mode(),
            KeyCode::Char('/') => self.ui_state.toggle_search_mode(),

            // Handle filter/search input
            _ => {
                if self.ui_state.is_input_mode() {
                    self.ui_state.handle_input(key);
                }
            }
        }

        Ok(())
    }

    /// Refresh data from Google Cloud
    async fn refresh_data(&mut self) -> Result<()> {
        info!("Refreshing instance data...");

        // Get instances from cloud
        let instances = self
            .cloud_client
            .list_instances()
            .await
            .context("Failed to fetch instances")?;

        // Update UI state with new data
        self.ui_state.update_instances(instances);

        // Make sure we have a valid selection after updating instances
        if !self.ui_state.has_valid_selection() {
            self.ui_state.reset_selection();
        }

        // Update refresh time
        self.last_refresh = Instant::now();

        // Update UI info (region, project, version)
        self.update_ui_info();

        Ok(())
    }

    /// Update UI state with cloud client information
    fn update_ui_info(&mut self) {
        // Set project ID
        let project_id = self.cloud_client.get_project_id().to_string();

        // Set region
        let region = self.cloud_client.get_region().to_string();

        // Try to get CLI version
        let cli_version = match self.cloud_client.get_cli_version() {
            Ok(version) => version,
            Err(e) => {
                error!("Failed to get CLI version: {}", e);
                "Unknown".to_string()
            }
        };

        // Update UI state
        self.ui_state
            .update_cloud_info(project_id, region, cli_version);
    }

    /// Perform an action on an instance
    async fn perform_action(&mut self, action: Action, instance_id: String) -> Result<()> {
        info!("Performing action on instance {}", instance_id);

        // Perform the action
        match action {
            Action::Start => {
                self.cloud_client.start_instance(&instance_id).await?;
            }
            Action::Stop => {
                self.cloud_client.stop_instance(&instance_id).await?;
            }
            Action::Restart => {
                self.cloud_client.restart_instance(&instance_id).await?;
            }
        }

        // Refresh data after action
        self.refresh_data().await?;

        Ok(())
    }
}
