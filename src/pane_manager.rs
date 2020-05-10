extern crate easycurses;
use crate::Kanban;
use crate::Item;

use easycurses::{EasyCurses, Input, CursorVisibility, ColorPair, Color};

pub struct PaneManager {
    pub left_divider: i32,
    pub right_divider: i32,
    pub selected_pane: i32,
    pub selected_item: i32,
    pub bottom_divider: i32
}

impl PaneManager {
    pub fn new(row_count: i32, col_count: i32) -> PaneManager {
        let left_loc = col_count / 3;
        let right_loc = (col_count / 3) * 2;

        return PaneManager {
            left_divider: left_loc,
            right_divider: right_loc,
            selected_pane: 1,
            selected_item: 0,
            bottom_divider: row_count - 2
        };
    }

    pub fn set_selected_pane(&mut self, s: i32) {
        self.selected_pane = s;
    }

    pub fn set_selected_item(&mut self, s: i32) {
        self.selected_item = s;
    }

    fn render_lists(&mut self, easy: &mut EasyCurses, kanban: &mut Kanban) {
        for (pos, i) in (&kanban.todo).iter().enumerate() {
            easy.move_rc((pos as i32) + 2, 1);

            if pos == self.selected_item as usize && self.selected_pane == 1 {
                easy.print(format!("> {} ", i.name));
            } else {
                easy.print(format!("  {} ", i.name));
            }
        }

        for (pos, i) in (&kanban.working).iter().enumerate() {
            easy.move_rc((pos as i32 + 2), self.left_divider + 2);

            if pos == self.selected_item as usize && self.selected_pane == 2 {
                easy.print(format!("> {} ", i.name));
            } else {
                easy.print(format!("  {} ", i.name));
            }
        }

        for (pos, i) in (&kanban.done).iter().enumerate() {
            easy.move_rc((pos as i32) + 2, self.right_divider + 2);

            if pos == self.selected_item as usize && self.selected_pane == 3 {
                easy.print(format!("> {} ", i.name));
            } else {
                easy.print(format!("  {} ", i.name));
            }
        }
    }

    fn render_panes(&self, easy: &mut EasyCurses) {
        let (row_count, col_count) = easy.get_row_col_count();

        // render vertical dividers
        for i in 0..row_count - 1 {
            easy.move_rc(i, self.left_divider);
            easy.print("\u{2503}");
            easy.move_rc(i, self.right_divider);
            easy.print("\u{2503}");
        }

        // render bottom divider
        easy.move_rc(self.bottom_divider, 0);
        for i in 0..col_count {
            if i == self.left_divider + 1 || i == self.right_divider + 1 {
                easy.print("\u{253B}");
            } else {
                easy.print("\u{2501}");
            }
            easy.move_rc(self.bottom_divider, i);
        }

        // render top divider
        easy.move_rc(1, 0);
        for i in 0..col_count {
            if i == self.left_divider + 1 || i == self.right_divider + 1 {
                easy.print("\u{254B}");
            } else {
                easy.print("\u{2501}");
            }
            easy.move_rc(1, i);
        }
    }

    pub fn render(&mut self, easy: &mut EasyCurses, kanban: &mut Kanban) {
        let (row_count, col_count) = easy.get_row_col_count();

        let todo_title = String::from("Todo");
        easy.move_rc(0, self.left_divider / 2 - (todo_title.len() as i32) / 2);
        easy.print(todo_title);

        let working_title = String::from("In progress");
        easy.move_rc(0, (self.left_divider + self.right_divider) / 2 - (working_title.len() as
            i32) / 2);
        easy.print("Working");

        let done_title = String::from("Done");
        easy.move_rc(0, (self.right_divider + col_count) / 2 - (done_title.len() as i32) / 2);
        easy.print("Done");

        self.render_lists(easy, kanban);
        self.render_panes(easy);

        match easy.get_input().unwrap() {
            Input::KeyLeft => {
                if self.selected_pane > 1 {
                    self.selected_pane -= 1;
                    self.selected_item = 0;
                }
            },
            Input::KeyDown => {
                self.selected_item += 1;
            }
            Input::KeyUp => {
                self.selected_item -= 1;
            }
            Input::KeyRight => {
                if self.selected_pane < 3 {
                    self.selected_pane += 1;
                    self.selected_item = 0;
                }
            },
            Input::Character(c) => {
                match c {
                    'i' => {
                        let insert_prompt = String::from(" New item: ");
                        let mut inp = String::new();

                        easy.set_cursor_visibility(CursorVisibility::Visible);
                        easy.set_input_mode(easycurses::InputMode::Cooked);
                        loop {
                            easy.move_rc(row_count - 1, 0);
                            easy.print(format!("{}{}", insert_prompt, inp));
                            easy.move_rc(row_count - 1, (inp.len() + insert_prompt.len()) as i32);

                            let input_char = easy.get_input().unwrap();
                            match input_char {
                                Input::Character('\n') => {
                                    easy.insert_line();
                                    easy.set_cursor_visibility(CursorVisibility::Invisible);
                                    break
                                }
                                Input::Character(c) => {
                                    inp.push(c);
                                }
                                _ => {}
                            }
                        }
                        easy.set_input_mode(easycurses::InputMode::Character);
                        kanban.todo.push(Item{name: inp});
                    },
                    _ => {}
                }
            }
            _ => {}
        }
        // easy.refresh();
    }
}