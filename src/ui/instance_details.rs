use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, Wrap},
    Frame,
    backend::Backend,
};

use crate::cloud::Instance;

/// Render the instance details popup
pub fn render<B: Backend>(frame: &mut Frame<B>, instance: &Instance, area: Rect) {
    // Create a centered popup
    let popup_area = create_centered_rect(80, 80, area);
    
    // Create a block for the popup
    let _block = Block::default()
        .title(format!("Instance Details: {}", instance.name))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));
    
    // Render the block
    frame.render_widget(Block::default().style(Style::default().bg(Color::Black)), popup_area);
    
    // Split the popup into sections
    let popup_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1),  // Title
            Constraint::Length(12), // Basic info table
            Constraint::Min(3),     // Description and metadata
            Constraint::Length(1),  // Status line
        ])
        .split(popup_area);
    
    // Render title
    // Get status emoji
    let status_emoji = match instance.status.as_str() {
        "RUNNING" => "🟢",
        "TERMINATED" => "🔴",
        "STOPPING" => "🟠",
        "PROVISIONING" => "🟡",
        "STAGING" => "🔄",
        "SUSPENDED" => "💤",
        "REPAIRING" => "🟡",
        "PENDING" => "🟡",
        _ => "❓",
    };
    
    let title = Paragraph::new(Spans::from(vec![
        Span::styled(
            format!("Instance: {} ({}) {}", instance.name, instance.id, status_emoji),
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ),
    ]));
    frame.render_widget(title, popup_chunks[0]);
    
    // Render basic info table
    render_basic_info(frame, instance, popup_chunks[1]);
    
    // Render description and metadata
    render_metadata(frame, instance, popup_chunks[2]);
    
    // Render status line
    let status_line = Paragraph::new(Spans::from(vec![
        Span::raw("Press "),
        Span::styled("ESC", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to close, "),
        Span::styled("s", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to start, "),
        Span::styled("S", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to stop, "),
        Span::styled("R", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to restart"),
    ]));
    frame.render_widget(status_line, popup_chunks[3]);
}

/// Render the basic information table
fn render_basic_info<B: Backend>(frame: &mut Frame<B>, instance: &Instance, area: Rect) {
    // Get status emoji
    let status_emoji = match instance.status.as_str() {
        "RUNNING" => "🟢",
        "TERMINATED" => "🔴",
        "STOPPING" => "🟠",
        "PROVISIONING" => "🟡",
        "STAGING" => "🔄",
        "SUSPENDED" => "💤",
        "REPAIRING" => "🟡",
        "PENDING" => "🟡",
        _ => "❓",
    };
    
    let rows = vec![
        Row::new(vec![
            Cell::from("Status"),
            Cell::from(Span::styled(
                format!("{} {}", status_emoji, instance.status.clone()),
                status_style(&instance.status),
            )),
        ]),
        Row::new(vec![
            Cell::from("Machine Type"),
            Cell::from(instance.machine_type.clone()),
        ]),
        Row::new(vec![
            Cell::from("Zone"),
            Cell::from(instance.zone.clone()),
        ]),
        Row::new(vec![
            Cell::from("External IP"),
            Cell::from(instance.external_ip.clone().unwrap_or_else(|| "None".into())),
        ]),
        Row::new(vec![
            Cell::from("Internal IP"),
            Cell::from(instance.internal_ip.clone().unwrap_or_else(|| "None".into())),
        ]),
        Row::new(vec![
            Cell::from("Created"),
            Cell::from(instance.creation_timestamp.clone().unwrap_or_else(|| "Unknown".into())),
        ]),
    ];

    let table = Table::new(rows)
        .block(Block::default().borders(Borders::ALL).title("Basic Info"))
        .header(Row::new(vec![
            Cell::from(Span::styled(
                "Property",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Cell::from(Span::styled(
                "Value",
                Style::default().add_modifier(Modifier::BOLD),
            )),
        ]))
        .widths(&[Constraint::Percentage(30), Constraint::Percentage(70)])
        .column_spacing(1);

    frame.render_widget(table, area);
}

/// Render metadata and description
fn render_metadata<B: Backend>(frame: &mut Frame<B>, instance: &Instance, area: Rect) {
    // Split area into description and metadata
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),  // Description
            Constraint::Min(0),     // Metadata
        ])
        .split(area);
    
    // Render description if available
    let description = instance.description.clone().unwrap_or_else(|| "No description available".into());
    let description_paragraph = Paragraph::new(description)
        .block(Block::default().borders(Borders::ALL).title("Description"))
        .wrap(Wrap { trim: true });
    frame.render_widget(description_paragraph, chunks[0]);
    
    // Render metadata if available
    let metadata_text = if let Some(metadata) = &instance.metadata {
        format!("{:#?}", metadata)
    } else {
        "No metadata available".to_string()
    };
    
    let metadata_paragraph = Paragraph::new(metadata_text)
        .block(Block::default().borders(Borders::ALL).title("Metadata"))
        .wrap(Wrap { trim: true });
    frame.render_widget(metadata_paragraph, chunks[1]);
}

/// Get the style for a status
fn status_style(status: &str) -> Style {
    match status {
        "RUNNING" => Style::default().fg(Color::Green),
        "TERMINATED" => Style::default().fg(Color::Red),
        "STOPPING" => Style::default().fg(Color::Yellow),
        "PROVISIONING" => Style::default().fg(Color::Yellow),
        "STAGING" => Style::default().fg(Color::Cyan),
        "SUSPENDED" => Style::default().fg(Color::Gray),
        "REPAIRING" => Style::default().fg(Color::Yellow),
        "PENDING" => Style::default().fg(Color::Yellow),
        _ => Style::default(),
    }
}

/// Helper function to create a centered rect
fn create_centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}