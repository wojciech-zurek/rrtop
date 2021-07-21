use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::symbols::line;
use tui::text::Span;
use tui::widgets::{Block, Borders, LineGauge, Widget};

use crate::colorscheme::theme::Theme;
use crate::metric::Metric;
use crate::update::Updatable;
use crate::widget::title_span;

pub struct HitRate<'a> {
    title: String,
    theme: &'a Theme,
    hit_rate: f64,
}

impl<'a> HitRate<'a> {
    pub fn new(theme: &'a Theme) -> Self {
        HitRate {
            title: "hit rate".to_owned(),
            theme,
            hit_rate: 0.0,
        }
    }
}

impl<'a> Widget for &HitRate<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::default()
            .borders(Borders::ALL)
            .border_style(self.theme.hit_rate_border)
            .title(title_span(&self.title, self.theme.hit_rate_title, self.theme.hit_rate_border))
            .render(area, buf);

        let gauge = LineGauge::default()
            .gauge_style(self.theme.hit_rate_gauge)
            .label(Span::styled(format!("{:.2}%", self.hit_rate * 100.0), self.theme.hit_rate_label))
            .line_set(line::THICK)
            .ratio(self.hit_rate);
        gauge.render(Rect::new(area.x + 2, area.y + 1, area.width - 4, area.height), buf);
    }
}

impl<'a> Updatable<&Metric> for HitRate<'a> {
    fn update(&mut self, metric: &Metric) {
        self.hit_rate = metric.keyspace.keyspace_hit_rate;
    }
}