//! Settings modal rendering with sectioned layout

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use crate::studio::state::{SettingsField, SettingsSection, SettingsState};
use crate::theme;
use crate::theme::names::{gradients, tokens};

/// Unicode box drawing characters for visual polish
const BOX_HORIZONTAL: &str = "─";

pub fn render(frame: &mut Frame, area: Rect, state: &SettingsState) {
    frame.render_widget(Clear, area);

    let t = theme::current();

    // Title with modification indicator
    let title = if state.modified {
        " Settings * "
    } else {
        " Settings "
    };

    let block = Block::default()
        .title(title)
        .title_style(
            Style::default()
                .fg(Color::from(t.color(tokens::TEXT_PRIMARY)))
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::from(t.color(tokens::BORDER_FOCUSED))));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Layout: settings fields, theme preview strip, footer
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),    // Settings fields
            Constraint::Length(2), // Theme preview strip
            Constraint::Length(3), // Footer
        ])
        .split(inner);

    render_settings_fields(frame, chunks[0], state);
    render_theme_strip(frame, chunks[1]);
    render_footer(frame, chunks[2], state);
}

fn render_settings_fields(frame: &mut Frame, area: Rect, state: &SettingsState) {
    let t = theme::current();
    let mut lines = Vec::new();
    let mut current_section: Option<SettingsSection> = None;

    for (idx, field) in SettingsField::all().iter().enumerate() {
        let section = field.section();

        // Section header when section changes
        if current_section != Some(section) {
            if current_section.is_some() {
                lines.push(Line::from("")); // Spacing between sections
            }

            let section_name = section.display_name();
            lines.push(Line::from(Span::styled(
                section_name,
                Style::default()
                    .fg(Color::from(t.color(tokens::ACCENT_PRIMARY)))
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(Span::styled(
                BOX_HORIZONTAL.repeat(section_name.len()),
                Style::default().fg(Color::from(t.color(tokens::TEXT_DIM))),
            )));

            current_section = Some(section);
        }

        let is_selected = idx == state.selected_field;
        let value = state.get_field_value(*field);

        // Styles based on selection
        let (label_style, value_style, row_style) = if is_selected {
            (
                Style::default()
                    .fg(Color::from(t.color(tokens::ACCENT_SECONDARY)))
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::from(t.color(tokens::BG_HIGHLIGHT))),
                Style::default()
                    .fg(Color::from(t.color(tokens::TEXT_PRIMARY)))
                    .bg(Color::from(t.color(tokens::BG_HIGHLIGHT))),
                Style::default().bg(Color::from(t.color(tokens::BG_HIGHLIGHT))),
            )
        } else {
            (
                Style::default().fg(Color::from(t.color(tokens::TEXT_SECONDARY))),
                Style::default().fg(Color::from(t.color(tokens::TEXT_MUTED))),
                Style::default(),
            )
        };

        // Show input buffer when editing
        let display_value = if state.editing && is_selected {
            match field {
                SettingsField::ApiKey => format!("{}█", "*".repeat(state.input_buffer.len())),
                _ => format!("{}█", state.input_buffer),
            }
        } else {
            value
        };

        // Light theme indicator
        let suffix = if *field == SettingsField::Theme {
            state
                .current_theme_info()
                .map_or("", |info| if info.variant == "light" { " ☀" } else { "" })
        } else {
            ""
        };

        // Build row with fixed-width label
        let label_width = 14;
        let label = format!("  {:width$}", field.display_name(), width = label_width);
        let value_text = format!("{}{}", display_value, suffix);

        // Pad to fill background highlight
        let padding_len = area
            .width
            .saturating_sub(label.len() as u16 + value_text.len() as u16 + 1);
        let padding = " ".repeat(padding_len as usize);

        lines.push(Line::from(vec![
            Span::styled(label, label_style),
            Span::styled(value_text, value_style),
            Span::styled(padding, row_style),
        ]));
    }

    // Error message
    if let Some(error) = &state.error {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            format!("  {}", error),
            Style::default().fg(Color::from(t.color(tokens::ERROR))),
        )));
    }

    frame.render_widget(Paragraph::new(lines), area);
}

#[allow(clippy::cast_precision_loss)]
fn render_theme_strip(frame: &mut Frame, area: Rect) {
    let t = theme::current();

    // Compact preview: palette swatches + gradient on one line
    let mut spans = vec![
        Span::styled("  ", Style::default()),
        Span::styled(
            "██",
            Style::default().fg(Color::from(t.color(tokens::ACCENT_PRIMARY))),
        ),
        Span::styled(" ", Style::default()),
        Span::styled(
            "██",
            Style::default().fg(Color::from(t.color(tokens::ACCENT_SECONDARY))),
        ),
        Span::styled(" ", Style::default()),
        Span::styled(
            "██",
            Style::default().fg(Color::from(t.color(tokens::ACCENT_TERTIARY))),
        ),
        Span::styled(" ", Style::default()),
        Span::styled(
            "██",
            Style::default().fg(Color::from(t.color(tokens::SUCCESS))),
        ),
        Span::styled(" ", Style::default()),
        Span::styled(
            "██",
            Style::default().fg(Color::from(t.color(tokens::WARNING))),
        ),
        Span::styled(" ", Style::default()),
        Span::styled(
            "██",
            Style::default().fg(Color::from(t.color(tokens::ERROR))),
        ),
        Span::styled(
            "  │  ",
            Style::default().fg(Color::from(t.color(tokens::TEXT_DIM))),
        ),
    ];

    // Add gradient
    let gradient_width = 24;
    for i in 0..gradient_width {
        let t_pos = i as f32 / (gradient_width - 1) as f32;
        let color = Color::from(t.gradient(gradients::PRIMARY, t_pos));
        spans.push(Span::styled("▀", Style::default().fg(color)));
    }

    let lines = vec![
        Line::from(Span::styled(
            BOX_HORIZONTAL.repeat(area.width as usize),
            Style::default().fg(Color::from(t.color(tokens::TEXT_DIM))),
        )),
        Line::from(spans),
    ];

    frame.render_widget(Paragraph::new(lines), area);
}

fn render_footer(frame: &mut Frame, area: Rect, state: &SettingsState) {
    let t = theme::current();

    let separator = Line::from(Span::styled(
        BOX_HORIZONTAL.repeat(area.width as usize),
        Style::default().fg(Color::from(t.color(tokens::TEXT_DIM))),
    ));

    let hints = if state.editing {
        Line::from(vec![
            Span::styled(
                "  Enter",
                Style::default().fg(Color::from(t.color(tokens::SUCCESS))),
            ),
            Span::styled(
                " confirm  ",
                Style::default().fg(Color::from(t.color(tokens::TEXT_MUTED))),
            ),
            Span::styled(
                "Esc",
                Style::default().fg(Color::from(t.color(tokens::WARNING))),
            ),
            Span::styled(
                " cancel",
                Style::default().fg(Color::from(t.color(tokens::TEXT_MUTED))),
            ),
        ])
    } else {
        Line::from(vec![
            Span::styled(
                "  ↑↓",
                Style::default().fg(Color::from(t.color(tokens::ACCENT_PRIMARY))),
            ),
            Span::styled(
                " nav  ",
                Style::default().fg(Color::from(t.color(tokens::TEXT_MUTED))),
            ),
            Span::styled(
                "←→",
                Style::default().fg(Color::from(t.color(tokens::ACCENT_PRIMARY))),
            ),
            Span::styled(
                " cycle  ",
                Style::default().fg(Color::from(t.color(tokens::TEXT_MUTED))),
            ),
            Span::styled(
                "Enter",
                Style::default().fg(Color::from(t.color(tokens::ACCENT_PRIMARY))),
            ),
            Span::styled(
                " edit  ",
                Style::default().fg(Color::from(t.color(tokens::TEXT_MUTED))),
            ),
            Span::styled(
                "s",
                Style::default()
                    .fg(Color::from(t.color(tokens::SUCCESS)))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " save  ",
                Style::default().fg(Color::from(t.color(tokens::TEXT_MUTED))),
            ),
            Span::styled(
                "Esc",
                Style::default().fg(Color::from(t.color(tokens::WARNING))),
            ),
            Span::styled(
                " close",
                Style::default().fg(Color::from(t.color(tokens::TEXT_MUTED))),
            ),
        ])
    };

    frame.render_widget(Paragraph::new(vec![separator, Line::from(""), hints]), area);
}
