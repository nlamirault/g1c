use std::io;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

mod dashboard;
mod instance_details;
mod help;
mod styles;

use crate::cloud::Instance;

/// UI state and action types
#[derive(Debug)]
pub enum Action {
    Start,
    Stop,
    Restart,
    Delete,
    Ssh,
    None,
}

/// UI state that manages all UI components
pub struct UiState {
    /// The list of instances
    instances: Vec<Instance>,
    /// Currently selected instance index
    selected_index: usize,
    /// Whether to show the help popup
    show_help: bool,
    /// Whether to show instance details
    show_details: bool,
    /// Whether we're in filter mode
    filter_mode: bool,
    /// Current filter text
    filter: String,
    /// Whether we're in search mode
    search_mode: bool,
    /// Current search text
    search: String,
    /// Current popup confirmation state
    confirmation: Option<Action>,
    /// Project ID from cloud client
    project_id: String,
    /// Region from cloud client
    region: String,
    /// gcloud CLI version
    cli_version: String,
}

impl UiState {
    /// Create a new UI state
    pub fn new() -> Self {
        Self {
            instances: Vec::new(),
            selected_index: 0,
            show_help: false,
            show_details: false,
            filter_mode: false,
            filter: String::new(),
            search_mode: false,
            search: String::new(),
            confirmation: None,
            project_id: String::new(),
            region: String::new(),
            cli_version: String::new(),
        }
    }
    
    /// Update cloud information
    pub fn update_cloud_info(&mut self, project_id: String, region: String, cli_version: String) {
        self.project_id = project_id;
        self.region = region;
        self.cli_version = cli_version;
    }

    /// Update the list of instances
    pub fn update_instances(&mut self, instances: Vec<Instance>) {
        let _old_len = self.instances.len();
        self.instances = instances;
        
        // Apply any active filters
        if !self.filter.is_empty() {
            self.apply_filter();
        }
        
        // Adjust selected index if needed
        self.ensure_valid_selection();
    }
    
    /// Apply the current filter to the instances
    fn apply_filter(&mut self) {
        let filter = self.filter.to_lowercase();
        self.instances.retain(|instance| {
            instance.name.to_lowercase().contains(&filter) || 
            instance.status.to_lowercase().contains(&filter) ||
            instance.machine_type.to_lowercase().contains(&filter) ||
            instance.zone.to_lowercase().contains(&filter) ||
            instance.network.as_ref().map_or(false, |n| n.to_lowercase().contains(&filter)) ||
            instance.internal_ip.as_ref().map_or(false, |ip| ip.to_lowercase().contains(&filter))
        });
        
        // Make sure selected index is still valid after filtering
        self.ensure_valid_selection();
    }
    
    /// Toggle help popup
    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
        self.filter_mode = false;
        self.search_mode = false;
    }
    
    /// Toggle filter mode
    pub fn toggle_filter_mode(&mut self) {
        self.filter_mode = !self.filter_mode;
        self.search_mode = false;
        if !self.filter_mode {
            // Reset filter when leaving filter mode
            self.filter.clear();
        }
    }
    
    /// Toggle search mode
    pub fn toggle_search_mode(&mut self) {
        self.search_mode = !self.search_mode;
        self.filter_mode = false;
        if !self.search_mode {
            // Reset search when leaving search mode
            self.search.clear();
        }
    }
    
    /// Check if we're in any input mode (filter or search)
    pub fn is_input_mode(&self) -> bool {
        self.filter_mode || self.search_mode
    }
    
    /// Handle input in filter or search mode
    pub fn handle_input(&mut self, key: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;
        
        let input_str = match key.code {
            KeyCode::Char(c) => Some(c.to_string()),
            KeyCode::Backspace => {
                let input = if self.filter_mode { &mut self.filter } else { &mut self.search };
                if !input.is_empty() {
                    input.pop();
                }
                None
            },
            _ => None,
        };
        
        if let Some(s) = input_str {
            if self.filter_mode {
                self.filter.push_str(&s);
            } else if self.search_mode {
                self.search.push_str(&s);
            }
        }
    }
    
    /// Show details for the selected instance
    pub fn show_details(&mut self) {
        if !self.instances.is_empty() {
            self.show_details = true;
        }
    }
    
    /// Close any open popup
    pub fn close_popup(&mut self) {
        self.show_help = false;
        self.show_details = false;
        self.filter_mode = false;
        self.search_mode = false;
        self.confirmation = None;
    }
    
    /// Navigate to previous item in the list
    pub fn previous_item(&mut self) {
        if !self.instances.is_empty() {
            if self.selected_index > 0 {
                self.selected_index -= 1;
            } else {
                self.selected_index = self.instances.len() - 1;
            }
        }
    }
    
    /// Navigate to next item in the list
    pub fn next_item(&mut self) {
        if !self.instances.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.instances.len();
        }
    }
    
    /// Ensure the selected index is valid
    fn ensure_valid_selection(&mut self) {
        if !self.instances.is_empty() && self.selected_index >= self.instances.len() {
            self.selected_index = self.instances.len() - 1;
        }
    }
    
    /// Check if the current selection is valid
    pub fn has_valid_selection(&self) -> bool {
        !self.instances.is_empty() && self.selected_index < self.instances.len()
    }
    
    /// Reset selection to the first item if possible
    pub fn reset_selection(&mut self) {
        self.selected_index = if self.instances.is_empty() { 0 } else { 0 };
    }
    
    /// Get the ID of the currently selected instance
    pub fn selected_instance_id(&self) -> Option<String> {
        if self.instances.is_empty() {
            None
        } else {
            Some(self.instances[self.selected_index].id.clone())
        }
    }
    
    /// Show confirmation dialog for an action
    pub fn confirm_action(&mut self) -> bool {
        if let Some(_action) = self.confirmation.take() {
            true
        } else {
            false
        }
    }
}

/// Setup the terminal for TUI
pub fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend)
}

/// Restore terminal settings
pub fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}

/// Main render function that delegates to the appropriate view
pub fn render<B: Backend>(frame: &mut ratatui::Frame<B>, state: &UiState) {
    let size = frame.size();
    
    // Render the dashboard (main view)
    dashboard::render(frame, state, size);
    
    // Render popups if needed
    if state.show_help {
        help::render(frame, size);
    } else if state.show_details && !state.instances.is_empty() {
        let instance = &state.instances[state.selected_index];
        instance_details::render(frame, instance, size);
    }
}