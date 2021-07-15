use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::symbols;
use tui::text::{Span, Spans};
use tui::widgets::{Tabs, Widget};

use crate::colorscheme::theme::Theme;

pub struct Menu<'a, 'b> {
    pub titles: Vec<&'b str>,
    selected_tab: usize,
    theme: &'a Theme,
}

impl<'a, 'b> Menu<'a, 'b> {
    pub fn new(theme: &'a Theme, titles: Vec<&'b str>) -> Self {
        Menu {
            titles,
            selected_tab: 0,
            theme,
        }
    }

    pub fn select(&mut self, selected_tab: usize) {
        self.selected_tab = selected_tab
    }
}

impl<'a, 'b> Widget for &Menu<'a, 'b> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let titles = self
            .titles
            .iter()
            .map(|&it| {
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