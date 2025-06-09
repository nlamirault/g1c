use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

/// Render the help popup
pub fn render<B: Backend>(frame: &mut Frame<B>, area: Rect) {
    // Create a centered popup
    let popup_area = create_centered_rect(60, 70, area);

    // Create a block for the popup
    let block = Block::default()
        .title("Help")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    // Render the block
    frame.render_widget(
        Block::default().style(Style::default().bg(Color::Black)),
        popup_area,
    );

    // Create the help text
    let help_text = vec![
        Line::from(Span::styled(
            "GCI - Google Cloud Instances",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Navigation",
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Cyan),
        )),
        Line::from(vec![
            Span::styled("↑/k", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" - Move selection up"),
        ]),
        Line::from(vec![
            Span::styled("↓/j", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" - Move selection down"),
        ]),
        Line::from(vec![
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" - Show instance details"),
        ]),
        Line::from(vec![
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" - Close popup or cancel action"),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Filtering and Searching",
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Cyan),
        )),
        Line::from(vec![
            Span::styled("f", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" - Toggle filter mode"),
        ]),
        Line::from(vec![
            Span::styled("/", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" - Toggle search mode"),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Instance Actions",
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Cyan),
        )),
        Line::from(vec![
            Span::styled("s", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" - Start selected instance"),
        ]),
        Line::from(vec![
            Span::styled("S", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" - Stop selected instance"),
        ]),
        Line::from(vec![
            Span::styled("R", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" - Restart selected instance"),
        ]),
        Line::from(vec![
            Span::styled("d", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" - Delete selected instance (with confirmation)"),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Miscellaneous",
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Cyan),
        )),
        Line::from(vec![
            Span::styled("r", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" - Refresh instance data"),
        ]),
        Line::from(vec![
            Span::styled("?", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" - Toggle this help screen"),
        ]),
        Line::from(vec![
            Span::styled("q/Ctrl+c", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" - Quit application"),
        ]),
    ];

    let paragraph = Paragraph::new(help_text)
        .block(block)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, popup_area);
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
