use crate::colorscheme::theme::Theme;
use tui::layout::{Rect, Alignment};
use tui::buffer::Buffer;
use tui::widgets::{Widget, Borders, Block, Paragraph};
use tui::text::Span;
use crate::update::Updatable;
use crate::metric::Metric;

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

        let s = Span::from(format!("{}ms {} (v{}) pid:{}({})", self.latency, uptime, self.version, self.process_id, self.role));

        Paragraph::new(s)
            .alignment(Alignment::Right)
            .style(self.theme.status_bar)
            .render(area, buf);
    }
}

impl<'a> Updatable<&Metric> for StatusBar<'a> {
    fn update(&mut self, metric: &Metric) {
        self.version = metric.status.version.to_owned();
        self.uptime = metric.status.uptime;

        self.process_id = metric.status.process_id;

        self.role = metric.status.role.to_owned();
        self.latency = metric.status.latency;
    }
}
