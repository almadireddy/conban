extern crate easycurses;

mod pane_manager;

use easycurses::{EasyCurses, TimeoutMode, CursorVisibility, Input, InputMode, Color, ColorPair};
use std::fmt;
use std::process::exit;
use std::path::Component::CurDir;
use pane_manager::PaneManager;

#[derive(Debug)]
struct Item {
    name: String
}

#[derive(Debug)]
struct Kanban {
    pub todo: Vec<Item>,
    pub working: Vec<Item>,
    pub done: Vec<Item>,
}

fn initialize_sample_data() -> Kanban {
    let mut kanban: Kanban = Kanban {
        todo: vec![],
        working: vec![],
        done: vec![]
    };

    let mut todo_items: Vec<Item> = Vec::new();
    let mut working_items: Vec<Item> = Vec::new();
    for n in 0..3 {
        let i = Item{ name: format!("todo number {}", n)};
        let w = Item{ name: format!("working number {}", n)};

        todo_items.push(i);
        working_items.push(w);
    }

    kanban.todo = todo_items;
    kanban.working = working_items;

    return kanban;
}

fn main() {
    let mut easy = EasyCurses::initialize_system().unwrap();
    easy.set_cursor_visibility(CursorVisibility::Invisible);

    let (row_count, col_count) = easy.get_row_col_count();

    let mut kanban: Kanban = initialize_sample_data();

    easy.set_keypad_enabled(true);
    easy.set_input_mode(InputMode::Character);

    easy.set_color_pair(ColorPair::new(Color::White, Color::Black));
    easy.set_bold(false);

    let mut pane = PaneManager::new(row_count, col_count);

    loop {
        pane.render(&mut easy, &mut kanban);
    }
}