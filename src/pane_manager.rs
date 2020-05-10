extern crate easycurses;
use crate::Kanban;
use crate::Item;

use easycurses::{EasyCurses, Input, CursorVisibility};

pub struct PaneManager {
    pub left_divider: i32,
    pub right_divider: i32,
    pub selected_pane: i32,
    pub selected_item: i32
}

impl PaneManager {
    pub fn new(row_count: i32, col_count: i32) -> PaneManager {
        let left_loc = col_count / 3;
        let right_loc = (col_count / 3) * 2;

        return PaneManager {
            left_divider: left_loc,
            right_divider: right_loc,
            selected_pane: 0,
            selected_item: 0
        };
    }

    pub fn set_selected_pane(&mut self, s: i32) {
        self.selected_pane = s;
    }

    pub fn set_selected_item(&mut self, s: i32) {
        self.selected_item = s;
    }

    pub fn render(&mut self, easy: &mut EasyCurses, kanban: &mut Kanban) {
        let mut cur_loc: i32 = 0;
        let (row_count, col_count) = easy.get_row_col_count();

        for i in 0..row_count {
            easy.move_rc(i, self.left_divider);
            easy.print("|");
            easy.move_rc(i, self.right_divider);
            easy.print("|");
        }

        for i in &kanban.todo {
            easy.move_rc(cur_loc, 1);
            easy.print(i.name.as_str());
            cur_loc += 1;
        }

        cur_loc = 0;

        for i in &kanban.working {
            easy.move_rc(cur_loc, self.left_divider + 2);
            easy.print(i.name.as_str());
            cur_loc += 1;
        }

        cur_loc = 0;

        for i in &kanban.done {
            easy.move_rc(cur_loc, self.right_divider + 2);
            easy.print(i.name.as_str());
            cur_loc += 1;
        }

        easy.refresh();
        match easy.get_input().unwrap() {
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
            Input::Character(c) => {
                match c {
                    'i' => {
                        let mut inp = String::new();
                        let mut cur_inp_col = self.right_divider + 2;

                        loop {
                            easy.move_rc(1, cur_inp_col);
                            easy.print(&inp);
                            easy.move_rc(1, cur_inp_col + inp.len() as i32);
                            easy.print("_");

                            let input_char = easy.get_input().unwrap();
                            match input_char {
                                Input::Character('\n')=> {
                                    for _ in 0..inp.len() {
                                        easy.delete_line();
                                    }
                                    break
                                }
                                Input::Character(c) => {
                                    inp.push(c);
                                }
                                _ => {}
                            }

                            easy.move_rc(1, cur_inp_col);
                        }

                        kanban.todo.push(Item{name: inp});
                    },
                    _ => {}
                }
            }
            _ => {

            }
        }
    }
}
