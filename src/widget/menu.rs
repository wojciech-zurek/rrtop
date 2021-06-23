use tui::widgets::{Widget, Tabs};
use tui::layout::Rect;
use tui::buffer::Buffer;
use tui::text::{Span, Spans};
use tui::style::{Style, Modifier};

pub struct Menu {
    pub titles: Vec<String>,
    pub selected: usize,
}

impl Menu {
    pub fn new() -> Self {
        Menu {
            titles: vec!["Main".to_owned(), "Other".to_owned()],
            selected: 0
        }
    }

    pub fn select(&mut self, selected: usize){
        self.selected = selected
    }
}

impl Widget for &Menu {
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
            .select(self.selected)
            // .style(Style::default().fg(Color::Cyan))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::REVERSED)
                //    .bg(Color::Black),
            );

        tabs.render(area, buf);
    }
}