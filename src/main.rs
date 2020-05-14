extern crate easycurses;
extern crate dirs;
extern crate serde;
extern crate serde_json;
extern crate ctrlc;

mod pane_manager;

use easycurses::{EasyCurses, TimeoutMode, CursorVisibility, Input, InputMode, Color, ColorPair};
use std::fmt;
use std::process::exit;
use std::path::{PathBuf};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use pane_manager::PaneManager;
use serde::{Serialize, Deserialize};
use serde_json::{Result};

#[derive(Debug, Serialize, Deserialize)]
struct Item {
    name: String
}

#[derive(Debug, Deserialize, Serialize)]
struct Kanban {
    pub todo: Vec<Item>,
    pub working: Vec<Item>,
    pub done: Vec<Item>,
    pub last_deleted: Item
}

impl Kanban {
    pub fn new() -> Kanban {
        return Kanban {
            todo: Vec::new(),
            working: Vec::new(),
            done: Vec::new(),
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
    return return OpenOptions::new()
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

fn main() {
    let mut easy = EasyCurses::initialize_system().unwrap();
    easy.set_cursor_visibility(CursorVisibility::Invisible);

    let (row_count, col_count) = easy.get_row_col_count();

    let mut kanban: Kanban = load_kanban();

    easy.set_keypad_enabled(true);
    easy.set_input_mode(InputMode::Character);
    easy.set_color_pair(ColorPair::new(Color::White, Color::Black));
    easy.set_bold(false);

    let mut pane = PaneManager::new(row_count, col_count);

    loop {
        pane.render(&mut easy, &mut kanban);
    }
}