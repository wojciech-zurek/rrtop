use crate::colorscheme::ColorScheme;
use tui::layout::{Rect, Alignment};
use tui::buffer::Buffer;
use tui::widgets::{Widget, Borders, Block, Paragraph};
use tui::text::Span;
use crate::update::Updatable;
use crate::response::Info;

pub struct StatusBar<'a> {
    uptime: i64,
    process_id: i64,
    version: String,
    role: String,
    color_scheme: &'a ColorScheme,
}

impl<'a> StatusBar<'a> {
    pub fn new(color_scheme: &'a ColorScheme) -> Self {
        StatusBar {
            uptime: 0,
            process_id: 0,
            version: "".to_owned(),
            role: "".to_owned(),
            color_scheme,
        }
    }
}

impl<'a> Widget for &StatusBar<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let duration = chrono::Duration::seconds(self.uptime);
        let uptime = format!("{}d {:02}:{:02}:{:02}", duration.num_days(), duration.num_hours() % 24, duration.num_minutes() % 60, duration.num_seconds() % 60);

        let s = Span::from(format!("{} (v{}) pid:{}({})", uptime, self.version, self.process_id, self.role));

        // let border_style = Style::default().fg(NORD_3).add_modifier(Modifier::BOLD);

        let paragraph = Paragraph::new(s)
            .alignment(Alignment::Right)
            .block(
                Block::default()
                    //       .title(Span::styled(self.title.as_str(), title_style))
                    //   .border_style(border_style)
                    .borders(Borders::NONE)
            );
        paragraph.render(area, buf);
    }
}

impl<'a> Updatable<&Info> for StatusBar<'a> {
    fn update(&mut self, v: &Info) {
        if let Some(rv) = v.0.get("redis_version") {
            self.version = rv.to_owned();
        }

        if let Some(u) = v.0.get("uptime_in_seconds") {
            self.uptime = u.parse::<i64>().unwrap_or(0);
        }

        if let Some(pid) = v.0.get("process_id") {
            self.process_id = pid.parse::<i64>().unwrap_or(0);
        }

        if let Some(r) = v.0.get("role") {
            self.role = r.to_owned();
        }
    }
}
