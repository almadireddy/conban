pub struct PaneManager {
    pub left_divider: i32,
    pub right_divider: i32,
    pub selected_pane: i32,
    pub selected_item: i32,
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
}
