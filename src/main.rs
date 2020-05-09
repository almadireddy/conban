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
    println!("todo: {:?}", kanban.todo);

    easy.set_keypad_enabled(true);
    easy.set_input_mode(InputMode::Character);
    easy.set_scrolling(true);
    easy.set_echo(false);
    easy.set_color_pair(ColorPair::new(Color::White, Color::Black));
    easy.set_bold(false);

    let mut cur_loc = 0;

    let mut pane = PaneManager::new(row_count, col_count);

    loop {
        for i in 0..row_count {
            easy.move_rc(i, pane.left_divider);
            easy.print("|");
            easy.move_rc(i, pane.right_divider);
            easy.print("|");
        }

        for i in &kanban.todo {
            easy.move_rc(cur_loc, 1);
            easy.print(i.name.as_str());
            cur_loc += 1;
        }

        cur_loc = 0;

        for i in &kanban.working {
            easy.move_rc(cur_loc, pane.left_divider + 2);
            easy.print(i.name.as_str());
            cur_loc += 1;
        }

        cur_loc = 0;

        for i in &kanban.done {
            easy.move_rc(cur_loc, pane.right_divider + 2);
            easy.print(i.name.as_str());
            cur_loc += 1;
        }

        easy.refresh();
        let i = easy.get_input();

        match i {
            Some(input) => {
                match input {
                    Input::KeyLeft => {
                        kanban.todo.push(Item{
                            name: String::from("Key left pressed")
                        });
                    },
                    Input::KeyRight => {
                        kanban.working.push(Item{
                            name: String::from("Key right pressed")
                        });
                    },
                    Input::Character(q) => {
                        match q {
                            'q' => {
                                break
                            },
                            _ => {}
                        }
                    }
                    Input::KeyUp => {
                        let mut inp = String::new();
                        let mut cur_inp_col = pane.right_divider + 2;

                        loop {
                            easy.move_rc(1, cur_inp_col);
                            easy.print(&inp);
                            easy.move_rc(1, cur_inp_col + inp.len() as i32);
                            easy.print("_");

                            let input_char = easy.get_input().unwrap();
                            match input_char {
                                Input::Character(c) => {
                                    inp.push(c);
                                }
                                Input::KeyUp => {
                                    for i in inp.chars() {
                                        easy.delete_line();
                                    }
                                    break
                                }
                                _ => {

                                }
                            }

                            easy.move_rc(1, cur_inp_col);
                        }

                        kanban.todo.push(Item{name: inp});
                    }
                    _ => {

                    }
                }
            },

            _ => {}
        }
    }

    println!("Quitting");
}