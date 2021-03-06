use tui::buffer::Buffer;
use tui::layout::{Alignment, Rect};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph, Widget};

use crate::colorscheme::theme::Theme;
use crate::widget::title_span;

pub struct AreaWarning<'a> {
    title: String,
    theme: &'a Theme,
    min_width: u16,
    min_height: u16,
}

impl<'a> AreaWarning<'a> {
    pub fn new(theme: &'a Theme, min_width: u16, min_height: u16) -> Self {
        AreaWarning {
            title: "warning".to_string(),
            theme,
            min_width,
            min_height,
        }
    }
}

impl<'a> Widget for &AreaWarning<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let spans = vec![
            Spans::from(Span::styled(format!("Current width: {}, current height: {}", area.width, area.height), self.theme.warning_text_1)),
            Spans::from(Span::styled(format!("Minimum requirement: width: {}, height: {}", self.min_width, self.min_height), self.theme.warning_text_2))
        ];
        Paragraph::new(spans)
            .alignment(Alignment::Center)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(self.theme.warning_border)
                .title(title_span(&self.title, self.theme.warning_title, self.theme.warning_border)))
            .render(area, buf);
    }
}
