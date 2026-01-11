use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, List, Paragraph},
};

use crate::app::{App, CurrentScreen};

pub fn ui(frame: &mut Frame, app: &mut App) {
    let main_rect = Rect::new(0, 0, 85, frame.area().height);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Max(3), Constraint::Min(1)])
        .split(main_rect);

    let title = Paragraph::new(
        Line::styled(
            "Note / Frequency Converter",
            Style::default().fg(Color::Green),
        )
        .centered(),
    )
    .block(Block::bordered());
    frame.render_widget(title, chunks[0]);

    let items = match app.notes_sorted {
        true => app.notes_sorted(),
        false => app.notes_in_input_order(),
    };

    let mut names: Vec<String> = Vec::new();

    for item in items {
        names.push(format!(
            "{} : {:.2} Hz -> {:.2} Hz",
            item.note, item.from_freq, item.freq
        ));
    }

    let bind_style = Style::default().fg(Color::Blue).bold();

    let binds_parts = match app.current_screen {
        CurrentScreen::Main => {
            if app.notes_sorted {
                vec![
                    Span::styled(" <q> ", bind_style),
                    Span::raw("quit"),
                    Span::styled("  <n> ", bind_style),
                    Span::raw("new note"),
                    Span::styled("  <↑/↓> ", bind_style),
                    Span::raw("move up/down"),
                    Span::styled("  <d> ", bind_style),
                    Span::raw("delete selected"),
                    Span::styled("  <s> ", bind_style),
                    Span::raw("unsort list "),
                ]
            } else {
                vec![
                    Span::styled(" <q> ", bind_style),
                    Span::raw("quit"),
                    Span::styled("  <n> ", bind_style),
                    Span::raw("new note"),
                    Span::styled("  <↑/↓> ", bind_style),
                    Span::raw("move up/down"),
                    Span::styled("  <d> ", bind_style),
                    Span::raw("delete selected"),
                    Span::styled("  <s> ", bind_style),
                    Span::raw("sort list "),
                ]
            }
        }
        CurrentScreen::NoteInput => vec![
            Span::styled(" <q> ", bind_style),
            Span::raw("quit"),
            Span::styled(" <Esc> ", bind_style),
            Span::raw("cancel"),
            Span::styled("  <Enter> ", bind_style),
            Span::raw("Accept "),
        ],
    };

    let binds = Line::from(binds_parts).centered();

    let mut list_block = Block::bordered().title_bottom(binds);

    if app.notes_sorted {
        list_block = list_block.title(Span::styled("Sorted", Style::default().fg(Color::Blue)));
    }

    let list = List::new(names)
        .block(list_block)
        .highlight_style(Style::default().bg(Color::Yellow).fg(Color::Black));
    frame.render_stateful_widget(list, chunks[1], &mut app.note_list_state);

    if let CurrentScreen::NoteInput = &app.current_screen {
        let popup_rect = centered_rect(50, 30, main_rect);

        let popup_block = Block::new()
            .style(Style::default().bg(Color::DarkGray))
            .title("Input Frequency");

        let note_text = Paragraph::new(app.note_input.clone()).block(popup_block);

        frame.render_widget(note_text, popup_rect);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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
