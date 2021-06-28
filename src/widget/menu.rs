use tui::widgets::{Widget, Tabs};
use tui::layout::Rect;
use tui::buffer::Buffer;
use tui::text::{Span, Spans};
use tui::style::{Style, Modifier};
use crate::colorscheme::ColorScheme;

pub struct Menu<'a> {
    pub titles: Vec<String>,
    selected_tab: usize,
    color_scheme: &'a ColorScheme,
}

impl <'a>Menu<'a> {
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

impl <'a> Widget for & Menu<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let titles = self
            .titles
            .iter()
            .map(|t| {
                let (first, rest) = t.split_at(1);
                Spans::from(vec![
                    Span::styled(first, Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled(rest, Style::default()),
                ])
            })
            .collect();

        let tabs = Tabs::new(titles)
            //   .block(Block::default().borders(Borders::ALL).title("Tabs"))
            .select(self.selected_tab)
            // .style(Style::default().fg(Color::Cyan))
            .highlight_style(
                self.color_scheme.menu_highlight_style
            );

        tabs.render(area, buf);
    }
}