use tui::widgets::TableState;

pub trait Navigation  {
    fn state(&mut self) -> &mut TableState;
    fn len(&self) -> usize;

    fn next(&mut self) {
        let len = self.len();
        let next = Self::next_item(self.state(), len);
        self.state().select(Some(next));
    }

    fn prev(&mut self) {
        let len = self.len();
        let prev = Self::previous(self.state(), len);
        self.state().select(Some(prev));
    }

    fn next_item(state: &TableState, len: usize) -> usize {
        match state.selected() {
            Some(i) => {
                if i >= len - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        }
    }

    fn previous(state: &TableState, len: usize) -> usize {
        match state.selected() {
            Some(i) => {
                if i == 0 {
                    len - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        }
    }
}
