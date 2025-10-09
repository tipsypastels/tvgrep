use ratatui::widgets::ListState;

pub trait ListStateExt {
    fn select_prev_or_last(&mut self);
    fn select_next_or_first(&mut self, count: usize);
}

impl ListStateExt for ListState {
    fn select_prev_or_last(&mut self) {
        if self.selected().is_none_or(|s| s == 0) {
            self.select_last();
        } else {
            self.select_previous();
        }
    }

    fn select_next_or_first(&mut self, count: usize) {
        if self.selected().is_some_and(|s| s >= count - 1) {
            self.select_first();
        } else {
            self.select_next();
        }
    }
}
