extern crate easycurses;
use crate::{Kanban, open_conban_file};
use crate::Item;

use easycurses::{EasyCurses, Input, CursorVisibility, ColorPair, Color};
use std::io::Write;

pub struct PaneManager {
    pub vertical_dividers: Vec<i32>,
    pub selected_pane: i32,
    pub selected_item: i32,
    pub bottom_divider: i32,
    pub row_count: i32,
    pub col_count: i32,
}

impl PaneManager {
    pub fn new(row_count: i32, col_count: i32, kanban: &Kanban) -> PaneManager {
        let mut pane_m: PaneManager = PaneManager {
            vertical_dividers: vec![],
            selected_pane: 0,
            selected_item: 0,
            bottom_divider: row_count - 2,
            row_count,
            col_count
        };

        // find first pane with > 0 items.
        for (pos, i) in (kanban.lists).iter().enumerate() {
            let list = kanban.list_items.get(i).unwrap();

            if list.len() > 0 {
                pane_m.selected_pane = pos as i32;
                break;
            }
        }

        let length = kanban.lists.len();
        for i in 0..length as i32 - 1 {
            let col = col_count / length as i32 * (i + 1);
            pane_m.vertical_dividers.push(col);
        }

        return pane_m;
    }

    fn render_lists(&mut self, easy: &mut EasyCurses, kanban: &mut Kanban) {
        for (pos, s) in (&kanban.lists).iter().enumerate() {
            let list = kanban.list_items.get(s).unwrap();
            let current_col : i32;

            if pos == 0 {
                current_col = 1;
            } else {
                current_col = self.vertical_dividers.get(pos - 1).unwrap() + 2;
            }

            for (num, i) in (list).iter().enumerate() {
                // the position is subtracted because drawing the vertical bars of
                // the dividers causes the characters on the bars' right to be pushed down
                // by one. This subtraction allows the alignment to stay the same.
                easy.move_rc((num as i32) + 2, current_col - pos as i32);

                if num == self.selected_item as usize && self.selected_pane == pos as i32 {
                    easy.set_color_pair(ColorPair::new(Color::Magenta, Color::White));
                    easy.print(format!("{}", i.name));
                    easy.set_color_pair(ColorPair::new(Color::White, Color::Black));
                } else {
                    easy.print(format!("{}", i.name));
                }
            }
        }
    }

    fn render_panes(&self, easy: &mut EasyCurses) {
        // render vertical dividers
        for col in &self.vertical_dividers {
            for x in 0..self.row_count - 1 {
                easy.move_rc(x, *col);
                easy.insert_char(easycurses::constants::acs::vline());
            }
        }

        easy.move_rc(self.bottom_divider, 0);
        for i in 0..self.col_count {
            // render bottom
            easy.move_rc(self.bottom_divider, i);
            if self.vertical_dividers.contains(&i) {
                easy.insert_char(easycurses::constants::acs::btee());
            } else {
                easy.insert_char(easycurses::constants::acs::hline());
            }

            // render top
            easy.move_rc(1, i);
            if self.vertical_dividers.contains(&i) {
                easy.insert_char(easycurses::constants::acs::plus());
            } else {
                easy.insert_char(easycurses::constants::acs::hline());
            }
        }
    }

    pub fn recompute_dividers(&mut self, kanban: &mut Kanban) {
        let length = kanban.lists.len();
        let mut v: Vec<i32> = vec![];

        for i in 0..length as i32 - 1 {
            let col = self.col_count / length as i32 * (i + 1);
            v.push(col);
        }

        self.vertical_dividers = v;
    }

    fn render_titles(&mut self, easy: &mut EasyCurses, kanban: &mut Kanban) {
        let size_of_pane = self.col_count / kanban.lists.len() as i32;
        for (pos, l) in (&kanban.lists).iter().enumerate() {
            if pos == 0 {
                easy.move_rc(0, (size_of_pane / 2) -
                    (l.len() as i32 / 2));
            } else {
                easy.move_rc(0, (pos as i32 * size_of_pane) +
                    (size_of_pane / 2) -
                    (l.len() as i32/ 2) - (pos as i32 - 1));
                // subtracting pos at the end again to compensate for vertical bars as above
            }
            easy.print(format!("{}", l));
        }

        let last_deleted= format!("{} {}", "Last Deleted:", kanban.last_deleted.name);
        easy.move_rc(self.row_count-1 , self.col_count / 2 - (last_deleted.len() / 2) as i32);
        easy.print(last_deleted);
    }

    pub fn render(&mut self, easy: &mut EasyCurses, kanban: &mut Kanban) {
        easy.clear();
        self.render_titles(easy, kanban);
        self.render_lists(easy, kanban);
        self.render_panes(easy);

        match easy.get_input().unwrap() {
            Input::KeyLeft | Input::Character('h') => {
                if self.selected_pane > 0 {
                    let new_pane = self.selected_pane - 1;

                    let l = kanban.lists.get(new_pane as usize).unwrap();
                    let list = kanban.list_items.get(l).unwrap();
                    if list.len() > 0 {
                        self.selected_pane = new_pane;
                        self.selected_item = 0;
                    }
                }
            },
            Input::KeyDown | Input::Character('j') => {
                let list_name = kanban.lists.get(self.selected_pane as usize).unwrap();
                let list = kanban.list_items.get(list_name).unwrap();
                if self.selected_item < (list.len() - 1) as i32 {
                    self.selected_item += 1;
                }
            }
            Input::KeyUp | Input::Character('k') => {
                if self.selected_item > 0 {
                    self.selected_item -= 1;
                }
            }
            Input::KeyRight | Input::Character('l') => {
                if self.selected_pane < kanban.lists.len() as i32 - 1 {
                    let new_pane = self.selected_pane + 1;

                    let l = kanban.lists.get(new_pane as usize).unwrap();
                    let list = kanban.list_items.get(l).unwrap();
                    if list.len() > 0 {
                        self.selected_pane = new_pane;
                        self.selected_item = 0;
                    }
                }
            }
            Input::Character(c) => {
                match c {
                    'x' => {
                        let s: Item;
                        let selected_list = kanban
                            .lists
                            .get(self.selected_pane as usize)
                            .unwrap();
                        let selected_items = kanban
                            .list_items
                            .get_mut(selected_list)
                            .unwrap();

                        if selected_items.len() == 0 {
                            return
                        }

                        s = selected_items.remove(self.selected_item as usize);

                        kanban.last_deleted = s;
                    },
                    'X' => {
                        if kanban.lists.len() > 1 {
                            let name = kanban
                                .lists
                                .get(self.selected_pane as usize)
                                .unwrap();

                            kanban.list_items.remove(name);
                            kanban.lists.remove(self.selected_pane as usize);

                            if self.selected_pane > 0 {
                                self.selected_pane -= 1;
                            }
                        }
                    }
                    'w' => {
                        if self.selected_item > 0 {
                            let list_name = kanban
                                .lists
                                .get(self.selected_pane as usize)
                                .unwrap();

                            let items = kanban
                                .list_items
                                .get_mut(list_name)
                                .unwrap();
                            let s = items.remove(self.selected_item as usize);
                            items.insert(self.selected_item as usize - 1, s);
                            self.selected_item -= 1;
                        }
                    }
                    's' => {
                        let list_name = kanban
                            .lists
                            .get(self.selected_pane as usize)
                            .unwrap();
                        let items = kanban
                            .list_items
                            .get_mut(list_name)
                            .unwrap();

                        if self.selected_item < items.len() as i32 - 1 {
                            let s = items.remove(self.selected_item as usize);
                            items.insert(self.selected_item as usize + 1, s);
                            self.selected_item += 1;
                        }
                    }
                    x if x == 'a' || x == 'd' => {
                        let s: Item;
                        let mut changer : i32 = 1;
                        if x == 'a' {
                            if self.selected_pane == 0 {
                                return;
                            }
                            changer = -1;

                        } else if x == 'd' {
                            if self.selected_pane == kanban.lists.len() as i32 - 1 {
                                return
                            }
                            changer = 1;
                        }

                        {
                            let current_list = kanban
                                .lists
                                .get(self.selected_pane as usize)
                                .unwrap();

                            let current_items = kanban
                                .list_items
                                .get_mut(current_list)
                                .unwrap();

                            s = current_items.remove(self.selected_item as usize);
                        }

                        {
                            let next_list = kanban
                                .lists
                                .get((self.selected_pane + changer) as usize)
                                .unwrap();

                            let next_items = kanban
                                .list_items
                                .get_mut(next_list)
                                .unwrap();

                            next_items.push(s);
                            self.selected_item = next_items.len() as i32 - 1;
                        }
                        self.selected_pane += changer;
                    }
                    x if x =='i' || x == '+' || x == 'I' => {
                        let insert_prompt: String;
                        if x == 'i' {
                            insert_prompt = String::from(" New item: ");
                        } else {
                            insert_prompt = String::from(" New list: ");
                        }

                        let mut inp = String::new();

                        easy.set_cursor_visibility(CursorVisibility::Visible);
                        easy.set_input_mode(easycurses::InputMode::Cooked);
                        loop {
                            easy.move_rc(self.row_count - 1, 0);
                            easy.print(format!("{}{}", insert_prompt, inp));
                            easy.move_rc(self.row_count - 1,
                                         (inp.len() + insert_prompt.len()) as i32);

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
                        if x == 'i' {
                            let first_list = kanban.lists.get(0).unwrap();
                            let first_items = kanban
                                .list_items
                                .get_mut(first_list)
                                .unwrap();
                            first_items.push(Item{name: inp});
                        } else {
                            kanban.lists.push(inp.clone());
                            kanban.list_items.insert(inp, Vec::new());
                            self.recompute_dividers(kanban);
                        }
                    },
                    _ => {}
                }
            }
            _ => {}
        }

        let mut f = open_conban_file(true);
        let s = serde_json::to_string(&kanban).unwrap();
        f.write(s.as_bytes());
    }
}
