extern crate easycurses;
extern crate dirs;
extern crate serde;
extern crate serde_json;
extern crate tui;
mod pane_manager;

use std::fs::{File, OpenOptions};
use pane_manager::PaneManager;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use tui::{Terminal, Frame};
use tui::backend::CrosstermBackend;
use std::io;
use tui::widgets::{Widget, Block, Borders, List, Text, ListState};
use tui::layout::{Layout, Constraint, Direction, Rect};
use tui::{style::{Style, Color, Modifier}};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io::{stdout, Write, Read},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Kanban {
    pub lists: Vec<String>,
    pub list_items: HashMap<String, Vec<Item>>,
    pub last_deleted: Item
}

impl Kanban {
    pub fn new() -> Kanban {
        return Kanban {
            lists: Vec::new(),
            list_items: HashMap::new(),
            last_deleted: Item{name: String::from("<none>") }
        };
    }
}

fn open_conban_file(overwrite: bool) -> File {
    let mut path = dirs::home_dir().unwrap();
    path.push(".conban.json");

    if overwrite {
        return OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(path.as_path()).unwrap();
    }
    return OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path.as_path()).unwrap();
}

fn load_kanban() -> Kanban {
    let mut file = open_conban_file(false);
    let mut file_contents = String::new();
    let size_read = file.read_to_string(&mut file_contents).unwrap();

    if size_read == 0 {
        return Kanban::new();
    }

    let kanban: Kanban = serde_json::from_str(file_contents.as_str()).unwrap();

    return kanban;
}

enum Event<I> {
    Input(I),
    Tick,
}

fn main() -> Result<(), Box<dyn Error>> {
    // let mut easy = EasyCurses::initialize_system().unwrap();
    // easy.set_cursor_visibility(CursorVisibility::Invisible);
    //
    // let (row_count, col_count) = easy.get_row_col_count();
    let mut kanban: Kanban = load_kanban();
    //
    // easy.set_keypad_enabled(true);
    // easy.set_input_mode(InputMode::Character);
    // easy.set_color_pair(ColorPair::new(Color::White, Color::Black));
    // easy.set_bold(false);
    //
    // let mut pane = PaneManager::new(row_count,
    //                                 col_count,
    //                                 &kanban);
    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let (tx, rx) = mpsc::channel();

    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            // poll for tick rate duration, if no events, sent tick event.
            if event::poll(tick_rate - last_tick.elapsed()).unwrap() {
                if let CEvent::Key(key) = event::read().unwrap() {
                    tx.send(Event::Input(key)).unwrap();
                }
            }
            if last_tick.elapsed() >= tick_rate {
                tx.send(Event::Tick).unwrap();
                last_tick = Instant::now();
            }
        }
    });

    let mut selected = 0;
    let mut selected_list = 0;

    loop {
        terminal.draw(|mut f| {
            for (pos, list) in (&kanban.lists).iter().enumerate() {
                let first_list = list;
                let i = (&kanban.list_items).get(first_list).unwrap();
                let l = List::new(i.iter().map(|it|
                    Text::Raw(std::borrow::Cow::Borrowed(it.name.as_str()))
                )).block(Block::default().title(list.as_ref()).borders(Borders::TOP))
                    .style(Style::default().fg(Color::White))
                    .highlight_style(Style::default().modifier(Modifier::ITALIC))
                    .highlight_symbol(">> ");
                let area = Rect::new(pos as u16 * 50, 0, 49, 10);
                let mut state = ListState::default();
                if selected_list == pos {
                    state.select(Some(selected as usize));
                }
                f.render_stateful_widget(l, area, &mut state);
            }
        });
        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    execute!(
                        terminal.backend_mut(),
                        LeaveAlternateScreen,
                        DisableMouseCapture
                    )?;
                    terminal.show_cursor()?;
                    break;
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    let current_list = kanban.lists.get(selected).unwrap();
                    let current_list_items = kanban.list_items.get(current_list).unwrap();

                    if selected < current_list_items.len() - 1 {
                        selected += 1;
                    }
                }
                KeyCode::Up | KeyCode::Char('k')  => {
                    if selected > 0 {
                        selected -= 1;
                    }
                }
                KeyCode::Right | KeyCode::Char('l')  => {
                    if selected_list < (&kanban.lists).len() {
                        selected_list += 1;
                    }
                }
                KeyCode::Left | KeyCode::Char('h')  => {
                    if selected_list > 0 {
                        selected_list -= 1;
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
    // loop {
    //     pane.render(&mut easy, &mut kanban);
    // }
    Ok(())
}