use tui::widgets::{Widget, Tabs, Block, Borders};
use tui::layout::Rect;
use tui::buffer::Buffer;
use tui::text::{Span, Spans};
use tui::style::{Style, Modifier};
use crate::colorscheme::ColorScheme;
use tui::symbols;

pub struct Menu<'a> {
    pub titles: Vec<String>,
    selected_tab: usize,
    color_scheme: &'a ColorScheme,
}

impl<'a> Menu<'a> {
    pub fn new(color_scheme: &'a ColorScheme) -> Self {
        Menu {
            titles: vec!["Main".to_owned(), "Other".to_owned()],
            selected_tab: 0,
            color_scheme,
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
                Spans::from(Span::styled(it, self.color_scheme.menu))
            })
            .collect();

        let tabs = Tabs::new(titles)
            .select(self.selected_tab)
            .style(self.color_scheme.main)
            .divider(Span::styled(symbols::line::VERTICAL, self.color_scheme.menu_divider))
            .highlight_style(self.color_scheme.menu_highlight);

        tabs.render(area, buf);
    }
}