use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::symbols;
use tui::text::{Span, Spans};
use tui::widgets::{Tabs, Widget};

use crate::colorscheme::theme::Theme;

pub struct Menu<'a> {
    pub titles: Vec<String>,
    selected_tab: usize,
    theme: &'a Theme,
}

impl<'a> Menu<'a> {
    pub fn new(theme: &'a Theme) -> Self {
        Menu {
            titles: vec!["Main".to_owned(), "Cmd".to_owned(), "Stat".to_owned()],
            selected_tab: 0,
            theme,
        }
    }

    pub fn select(&mut self, selected_tab: usize) {
        self.selected_tab = selected_tab
    }
}

impl<'a> Widget for &Menu<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let titles = self
            .titles
            .iter()
            .map(|it| {
                Spans::from(Span::styled(it, self.theme.menu))
            })
            .collect();

        let tabs = Tabs::new(titles)
            .select(self.selected_tab)
            .divider(Span::styled(symbols::line::VERTICAL, self.theme.menu_divider))
            .highlight_style(self.theme.menu_highlight);

        tabs.render(area, buf);
    }
}