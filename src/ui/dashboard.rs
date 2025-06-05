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
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // For title and filter bar
            Constraint::Min(0),    // For instance list
            Constraint::Length(1), // For status bar
        ])
        .split(area);

    // Render title and filter bar
    render_title_bar(frame, state, chunks[0]);

    // Render instances list
    render_instance_list(frame, state, chunks[1]);

    // Render status bar
    render_status_bar(frame, state, chunks[2]);
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
    let title = Paragraph::new("Google Cloud Instances (G1C)").style(
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    );
    frame.render_widget(title, chunks[0]);

    // Filter bar
    let filter_text = if state.filter_mode {
        format!("Filter: {}", state.filter)
    } else if state.search_mode {
        format!("Search: {}", state.search)
    } else {
        "Press 'f' to filter, '/' to search".to_string()
    };

    let filter_style = if state.filter_mode || state.search_mode {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let filter_bar = Paragraph::new(filter_text).style(filter_style);

    frame.render_widget(filter_bar, chunks[1]);
}

/// Render the instance list
fn render_instance_list<B: Backend>(frame: &mut Frame<B>, state: &UiState, area: Rect) {
    // Create a block for the list
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Instances")
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

    // Create header row for the list
    let header = ListItem::new(Spans::from(vec![
        Span::styled(
            "NAME",
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Blue),
        ),
        Span::raw(" | "),
        Span::styled(
            "STATUS",
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Blue),
        ),
        Span::raw(" | "),
        Span::styled(
            "TYPE",
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Blue),
        ),
        Span::raw(" | "),
        Span::styled(
            "ZONE",
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Blue),
        ),
        Span::raw(" | "),
        Span::styled(
            "EXTERNAL IP",
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

        // Create list item
        let item = ListItem::new(Spans::from(vec![
            Span::raw(format!("{:<20}", instance.name)),
            Span::raw(" | "),
            Span::styled(
                format!("{:<10}", instance.status),
                Style::default().fg(status_color),
            ),
            Span::raw(" | "),
            Span::raw(format!("{:<15}", instance.machine_type)),
            Span::raw(" | "),
            Span::raw(format!("{:<15}", instance.zone)),
            Span::raw(" | "),
            Span::raw(format!(
                "{:<15}",
                instance.external_ip.as_deref().unwrap_or("-")
            )),
        ]));

        items.push(item);
    }

    // Create the List widget
    let list = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    // Render the list with the current selection
    frame.render_stateful_widget(
        list,
        area,
        &mut ratatui::widgets::ListState::default().with_selected(Some(state.selected_index)),
    );
}

/// Render the status bar
fn render_status_bar<B: Backend>(frame: &mut Frame<B>, state: &UiState, area: Rect) {
    let selected_text = if !state.instances.is_empty() {
        let instance = &state.instances[state.selected_index];
        format!("Selected: {} ({})", instance.name, instance.id)
    } else {
        "No instances selected".to_string()
    };

    let help_hint = "Press '?' for help";

    let text = Spans::from(vec![
        Span::raw(selected_text),
        Span::raw(" | "),
        Span::styled(help_hint, Style::default().fg(Color::DarkGray)),
    ]);

    let paragraph = Paragraph::new(text).style(Style::default().fg(Color::White));

    frame.render_widget(paragraph, area);
}
