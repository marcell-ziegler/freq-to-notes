use std::io::Stdout;

use crossterm::event::{self, Event, KeyCode};
use freq_to_notes::Note;
use ratatui::{Terminal, prelude::CrosstermBackend, widgets::ListState};

use crate::ui::ui;

#[derive(Debug)]
pub struct App {
    notes: Vec<Note>,
    pub running: bool,
    pub current_screen: CurrentScreen,
    pub notes_sorted: bool,
    pub note_list_state: ListState,
    pub note_list_sorted_state: ListState,
    pub note_input: String,
}
impl App {
    // Gets a reference to the notes in the app in the order they were input
    pub fn notes_in_input_order(&self) -> Vec<Note> {
        self.notes.clone()
    }
    // Get a sorted copy of the notes stored in the app
    pub fn notes_sorted(&self) -> Vec<Note> {
        let mut sorted = self.notes.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        sorted
    }

    pub fn sorted_index_to_real_index(&self, index: usize) -> Option<usize> {
        let sorted = self.notes_sorted();
        let target = sorted[index].from_freq;
        let eps = 1e-3;
        self.notes
            .iter()
            .position(|x| (x.from_freq - target).abs() < eps)
    }
}
impl Default for App {
    fn default() -> Self {
        App {
            notes: Vec::new(),
            running: true,
            current_screen: CurrentScreen::Main,
            notes_sorted: false,
            note_list_state: ListState::default(),
            note_list_sorted_state: ListState::default(),
            note_input: String::new(),
        }
    }
}

#[derive(Debug)]
pub enum CurrentScreen {
    Main,
    NoteInput,
}

pub fn run_app(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut App,
) -> color_eyre::Result<()> {
    while app.running {
        terminal.draw(|frame| ui(frame, app))?;

        if let Event::Key(key_event) = event::read()? {
            // Handle key events
            if key_event.code == KeyCode::Char('q') {
                app.running = false;
            }

            match app.current_screen {
                CurrentScreen::Main => match key_event.code {
                    KeyCode::Up => match app.notes_sorted {
                        true => app.note_list_sorted_state.select_previous(),
                        false => app.note_list_state.select_previous(),
                    },
                    KeyCode::Down => match app.notes_sorted {
                        true => app.note_list_sorted_state.select_next(),
                        false => app.note_list_state.select_next(),
                    },
                    KeyCode::Char('d') => match app.notes_sorted {
                        false => {
                            if let Some(i) = app.note_list_state.selected() {
                                app.notes.remove(i);
                            }
                        }
                        true => {
                            if let Some(i) = app.note_list_sorted_state.selected()
                                && let Some(i2) = app.sorted_index_to_real_index(i)
                            {
                                app.notes.remove(i2);
                            }
                        }
                    },
                    KeyCode::Char('n') => {
                        app.current_screen = CurrentScreen::NoteInput;
                    }
                    KeyCode::Char('s') => {
                        app.notes_sorted = !app.notes_sorted;
                    }
                    _ => {}
                },
                CurrentScreen::NoteInput => match key_event.code {
                    KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main;
                        app.note_input = String::new();
                    }
                    KeyCode::Enter => {
                        let note: Option<Note>;
                        if let Ok(val) = app.note_input.parse::<f64>() {
                            note = Some(Note::from_freq(val))
                        } else if let Ok(val) = app.note_input.parse::<u32>() {
                            note = Some(Note::from_freq(val as f64))
                        } else {
                            note = None
                        }

                        if let Some(n) = note {
                            app.notes.push(n);
                        }

                        app.current_screen = CurrentScreen::Main;
                        app.note_input = String::new();
                    }
                    KeyCode::Char(c) => {
                        if "1234567890.".contains(c) {
                            app.note_input.push(c);
                        }
                    }
                    KeyCode::Backspace => {
                        if !app.note_input.is_empty() {
                            app.note_input.pop();
                        }
                    }
                    _ => {}
                },
            }
        }
    }
    Ok(())
}
