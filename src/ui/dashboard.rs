use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use super::UiState;

/// Render the main dashboard view
pub fn render<B: Backend>(frame: &mut Frame<B>, state: &UiState, area: Rect) {
    // Create the layout
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),   // For title and filter bar
            Constraint::Length(8),   // For overview panel
            Constraint::Min(10),     // For instance list - use all remaining space
            Constraint::Length(1),   // For status bar
        ])
        .split(area);

    // Render title and filter bar
    render_title_bar(frame, state, main_chunks[0]);

    // Render overview panel
    render_overview_panel(frame, state, main_chunks[1]);

    // Render instances list - use all remaining space
    render_instance_list(frame, state, main_chunks[2]);

    // Render status bar
    render_status_bar(frame, state, main_chunks[3]);
}

/// Render the title and filter bar
fn render_title_bar<B: Backend>(frame: &mut Frame<B>, state: &UiState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // For title
            Constraint::Length(1), // For filter bar
        ])
        .split(area);

    // Title
    let title = Paragraph::new("🌩️  Google Cloud Instances (G1C)").style(
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    );
    frame.render_widget(title, chunks[0]);

    // Filter bar
    let filter_text = if state.filter_mode {
        format!("🔍 Filter: {}", state.filter)
    } else if state.search_mode {
        format!("🔎 Search: {}", state.search)
    } else {
        "🔍 Press 'f' to filter, '/' to search".to_string()
    };

    let filter_style = if state.filter_mode || state.search_mode {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let filter_bar = Paragraph::new(filter_text).style(filter_style);

    frame.render_widget(filter_bar, chunks[1]);
}

/// Render the overview panel
fn render_overview_panel<B: Backend>(frame: &mut Frame<B>, state: &UiState, area: Rect) {
    // Create a block for the overview panel
    let block = Block::default()
        .borders(Borders::ALL)
        .title("📈 Overview")
        .title_style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        );

    // Create the content
    let instance_count = state.instances.len();
    
    // Count instances by status
    let mut running_count = 0;
    let mut stopped_count = 0;
    let mut other_count = 0;
    
    for instance in &state.instances {
        match instance.status.as_str() {
            "RUNNING" => running_count += 1,
            "TERMINATED" => stopped_count += 1,
            _ => other_count += 1,
        }
    }
    
    let content = vec![
        Spans::from(vec![
            Span::styled("🔑 Project ID: ", Style::default().fg(Color::Blue)),
            Span::raw(&state.project_id),
        ]),
        Spans::from(vec![
            Span::styled("🌎 Region: ", Style::default().fg(Color::Blue)),
            Span::raw(&state.region),
        ]),
        Spans::from(vec![
            Span::styled("🖥️  gcloud CLI: ", Style::default().fg(Color::Blue)),
            Span::raw(&state.cli_version),
        ]),
        Spans::from(Span::raw("")),
        Spans::from(vec![
            Span::styled("📊 Total Instances: ", Style::default().fg(Color::Green)),
            Span::styled(
                instance_count.to_string(), 
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            ),
        ]),
        Spans::from(vec![
            Span::styled("  ✅ Running: ", Style::default().fg(Color::Green)),
            Span::styled(
                running_count.to_string(), 
                Style::default().fg(Color::Green)
            ),
            Span::raw("  "),
            Span::styled("⏹️ Stopped: ", Style::default().fg(Color::Red)),
            Span::styled(
                stopped_count.to_string(), 
                Style::default().fg(Color::Red)
            ),
            Span::raw("  "),
            Span::styled("⚠️ Other: ", Style::default().fg(Color::Yellow)),
            Span::styled(
                other_count.to_string(), 
                Style::default().fg(Color::Yellow)
            ),
        ]),
        Spans::from(Span::raw("")),
    ];

    let paragraph = Paragraph::new(content)
        .block(block)
        .alignment(ratatui::layout::Alignment::Left);

    frame.render_widget(paragraph, area);
}

/// Render the instance list
fn render_instance_list<B: Backend>(frame: &mut Frame<B>, state: &UiState, area: Rect) {
    // Create a block for the list - make sure to use all available space
    let block = Block::default()
        .borders(Borders::ALL)
        .title("💻 Instances List")
        .title_style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        );

    // If there are no instances, show a message
    if state.instances.is_empty() {
        let no_instances_text = vec![
            Spans::from(Span::styled(
                "No instances found",
                Style::default().fg(Color::Gray),
            )),
            Spans::from(Span::styled(
                "Press 'r' to refresh",
                Style::default().fg(Color::DarkGray),
            )),
        ];

        let paragraph = Paragraph::new(no_instances_text)
            .block(block)
            .alignment(ratatui::layout::Alignment::Center)
            .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, area);
        return;
    }

    // Calculate the available width for the table
    let available_width = area.width as usize - 16; // Subtract borders, margins, and column separators
    
    // Define column widths proportionally to available space
    let name_width = (available_width * 18) / 100;
    let status_width = (available_width * 10) / 100;
    let machine_type_width = (available_width * 18) / 100;
    let zone_width = (available_width * 15) / 100;
    let network_width = (available_width * 12) / 100;
    let internal_ip_width = (available_width * 14) / 100;
    let external_ip_width = (available_width * 13) / 100;

    // Create header row for the list
    let header = ListItem::new(Spans::from(vec![
        Span::styled(
            format!("{:<width$}", "NAME", width = name_width),
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Blue),
        ),
        Span::raw("│ "),
        Span::styled(
            format!("{:<width$}", "STATUS", width = status_width),
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Blue),
        ),
        Span::raw("│ "),
        Span::styled(
            format!("{:<width$}", "MACHINE TYPE", width = machine_type_width),
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Blue),
        ),
        Span::raw("│ "),
        Span::styled(
            format!("{:<width$}", "ZONE", width = zone_width),
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Blue),
        ),
        Span::raw("│ "),
        Span::styled(
            format!("{:<width$}", "NETWORK", width = network_width),
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Blue),
        ),
        Span::raw("│ "),
        Span::styled(
            format!("{:<width$}", "INTERNAL IP", width = internal_ip_width),
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Blue),
        ),
        Span::raw("│ "),
        Span::styled(
            format!("{:<width$}", "EXTERNAL IP", width = external_ip_width),
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Blue),
        ),
    ]));

    // Create list items from instances
    let mut items = vec![header];

    for (_i, instance) in state.instances.iter().enumerate() {
        // Determine status color
        let status_color = match instance.status.as_str() {
            "RUNNING" => Color::Green,
            "TERMINATED" => Color::Red,
            "STOPPING" => Color::Yellow,
            "PROVISIONING" => Color::Magenta,
            "STAGING" => Color::Cyan,
            _ => Color::Gray,
        };

        // Get network name (if available)
        let network = instance.network.as_deref().unwrap_or("-");
        
        // Format strings to limit length and avoid overflow
        let instance_name = if instance.name.len() > name_width {
            format!("{}…", &instance.name[0..name_width-1])
        } else {
            instance.name.clone()
        };
        
        let instance_status = instance.status.clone();
        
        let machine_type = if instance.machine_type.len() > machine_type_width {
            format!("{}…", &instance.machine_type[0..machine_type_width-1])
        } else {
            instance.machine_type.clone()
        };
        
        let zone = if instance.zone.len() > zone_width {
            format!("{}…", &instance.zone[0..zone_width-1])
        } else {
            instance.zone.clone()
        };
        
        let network_str = if network.len() > network_width {
            format!("{}…", &network[0..network_width-1])
        } else {
            network.to_string()
        };
        
        let internal_ip = instance.internal_ip.as_deref().unwrap_or("-").to_string();
        let external_ip = instance.external_ip.as_deref().unwrap_or("-").to_string();
            
        // Create list item with dynamic width columns
        let item = ListItem::new(Spans::from(vec![
            Span::raw(format!("{:<width$}", instance_name, width = name_width)),
            Span::raw("│ "),
            Span::styled(
                format!("{:<width$}", instance_status, width = status_width),
                Style::default().fg(status_color),
            ),
            Span::raw("│ "),
            Span::raw(format!("{:<width$}", machine_type, width = machine_type_width)),
            Span::raw("│ "),
            Span::raw(format!("{:<width$}", zone, width = zone_width)),
            Span::raw("│ "),
            Span::raw(format!("{:<width$}", network_str, width = network_width)),
            Span::raw("│ "),
            Span::raw(format!("{:<width$}", internal_ip, width = internal_ip_width)),
            Span::raw("│ "),
            Span::raw(format!("{:<width$}", external_ip, width = external_ip_width)),
        ]));

        items.push(item);
    }

    // Create a List widget - ensure it takes all available space
    let list = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ")
        .style(Style::default().fg(Color::White)); // Add default style for all list items

    // Create a ListState with the current selection
    let mut list_state = ratatui::widgets::ListState::default();
    list_state.select(Some(state.selected_index));
    
    // Render the list with the current selection, ensuring it uses all available space
    frame.render_stateful_widget(list, area, &mut list_state);
}

/// Render the status bar
fn render_status_bar<B: Backend>(frame: &mut Frame<B>, state: &UiState, area: Rect) {
    let selected_text = if !state.instances.is_empty() {
        let instance = &state.instances[state.selected_index];
        format!("🔍 Selected: {} ({})", instance.name, instance.id)
    } else {
        "🔍 No instances selected".to_string()
    };

    let help_hint = "❓ Press '?' for help";

    let text = Spans::from(vec![
        Span::raw(selected_text),
        Span::raw(" | "),
        Span::styled(help_hint, Style::default().fg(Color::DarkGray)),
    ]);

    let paragraph = Paragraph::new(text).style(Style::default().fg(Color::White));

    frame.render_widget(paragraph, area);
}
