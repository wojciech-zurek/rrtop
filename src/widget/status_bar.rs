use tui::buffer::Buffer;
use tui::layout::{Alignment, Rect};
use tui::text::Span;
use tui::widgets::{Paragraph, Widget};

use crate::colorscheme::theme::Theme;
use crate::metric::Metric;
use crate::update::Updatable;

pub struct StatusBar<'a> {
    uptime: i64,
    process_id: i64,
    latency: u128,
    version: String,
    role: String,
    theme: &'a Theme,
}

impl<'a> StatusBar<'a> {
    pub fn new(theme: &'a Theme) -> Self {
        StatusBar {
            uptime: 0,
            process_id: 0,
            latency: 0,
            version: "".to_owned(),
            role: "".to_owned(),
            theme,
        }
    }
}

impl<'a> Widget for &StatusBar<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let duration = chrono::Duration::seconds(self.uptime);

        let uptime = format!("{}d {:02}:{:02}:{:02}", duration.num_days(), duration.num_hours() % 24, duration.num_minutes() % 60, duration.num_seconds() % 60);

        let s = Span::from(format!("\u{21c5} {}ms \u{2191} {} (\u{2387} {}) pid:{}({})", self.latency, uptime, self.version, self.process_id, self.role));

        Paragraph::new(s)
            .alignment(Alignment::Right)
            .style(self.theme.status_bar)
            .render(area, buf);
    }
}

impl<'a> Updatable<&Metric> for StatusBar<'a> {
    fn update(&mut self, metric: &Metric) {
        self.version = metric.server.version.to_owned();
        self.uptime = metric.server.uptime;

        self.process_id = metric.server.process_id;

        self.role = metric.server.role.to_owned();
        self.latency = metric.server.latency;
    }
}
