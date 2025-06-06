use ratatui::style::{Color, Modifier, Style};

/// Theme for the application UI
pub struct Theme {
    /// Background color
    pub background: Color,
    /// Foreground (text) color
    pub foreground: Color,
    /// Primary highlight color
    pub primary: Color,
    /// Secondary highlight color
    pub secondary: Color,
    /// Success color
    pub success: Color,
    /// Warning color
    pub warning: Color,
    /// Error color
    pub error: Color,
    /// Info color
    pub info: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: Color::Black,
            foreground: Color::White,
            primary: Color::Cyan,
            secondary: Color::Yellow,
            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
            info: Color::Blue,
        }
    }
}

impl Theme {
    /// Create a new dark theme
    pub fn dark() -> Self {
        Self::default()
    }
    
    /// Create a new light theme
    pub fn light() -> Self {
        Self {
            background: Color::White,
            foreground: Color::Black,
            primary: Color::Blue,
            secondary: Color::Magenta,
            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
            info: Color::Cyan,
        }
    }
    
    /// Get title style
    pub fn title_style(&self) -> Style {
        Style::default()
            .fg(self.primary)
            .add_modifier(Modifier::BOLD)
    }
    
    /// Get header style
    pub fn header_style(&self) -> Style {
        Style::default()
            .fg(self.secondary)
            .add_modifier(Modifier::BOLD)
    }
    
    /// Get selected item style
    pub fn selected_style(&self) -> Style {
        Style::default()
            .bg(self.primary)
            .fg(self.background)
            .add_modifier(Modifier::BOLD)
    }
    
    /// Get highlight style
    pub fn highlight_style(&self) -> Style {
        Style::default().fg(self.primary)
    }
    
    /// Get status style based on value
    pub fn status_style(&self, status: &str) -> Style {
        match status.to_uppercase().as_str() {
            "RUNNING" => Style::default().fg(self.success),
            "TERMINATED" => Style::default().fg(self.error),
            "STOPPING" => Style::default().fg(self.warning),
            "PROVISIONING" => Style::default().fg(self.warning),
            "STAGING" => Style::default().fg(self.secondary),
            "SUSPENDED" => Style::default().fg(Color::DarkGray),
            "REPAIRING" => Style::default().fg(self.warning),
            "PENDING" => Style::default().fg(self.warning),
            _ => Style::default().fg(self.foreground),
        }
    }
    
    /// Get style for a block
    pub fn block_style(&self) -> Style {
        Style::default().fg(self.foreground).bg(self.background)
    }
    
    /// Get style for borders
    pub fn border_style(&self) -> Style {
        Style::default().fg(self.primary)
    }
    
    /// Get text input style
    pub fn input_style(&self) -> Style {
        Style::default().fg(self.secondary)
    }
    
    /// Get help text style
    pub fn help_style(&self) -> Style {
        Style::default().fg(Color::DarkGray)
    }
}