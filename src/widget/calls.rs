use std::collections::BTreeSet;

use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::symbols::line;
use tui::text::Span;
use tui::widgets::{Block, Borders, LineGauge, Widget};

use crate::colorscheme::theme::Theme;
use crate::metric::command::CmdStat;
use crate::metric::Metric;
use crate::update::Updatable;
use crate::widget::title_span;

pub struct Calls<'a> {
    title: String,
    theme: &'a Theme,
    stats: BTreeSet<CmdStat>,
    sum_calls: f64,
    sum_usec: f64,
    sum_usec_per_call: f64,
}

impl<'a> Calls<'a> {
    pub fn new(theme: &'a Theme) -> Self {
        Calls {
            title: "calls".to_owned(),
            theme,
            stats: BTreeSet::new(),
            sum_calls: 0.0,
            sum_usec: 0.0,
            sum_usec_per_call: 0.0,
        }
    }
}

impl<'a> Widget for &Calls<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::default()
            .borders(Borders::ALL)
            .border_style(self.theme.memory_border)
            .title(title_span(&self.title, self.theme.network_title, self.theme.network_border))
            .render(area, buf);

        let mut i = 2u16;
        self.stats.iter().enumerate().for_each({
            |it| {
                let calls = it.1.calls as f64 / self.sum_calls;
                LineGauge::default()
                    .gauge_style(self.theme.memory_used_memory_dataset)
                    .label(Span::styled(format!("{:>12} {:>6.2}% {:>12}", it.1.name, calls * 100.0, it.1.calls), self.theme.memory_used_memory_text))
                    .line_set(line::THICK)
                    .ratio(calls)
                    .render(Rect::new(area.x + 2, area.y + it.0 as u16 + 2, area.width - 4, 1), buf);
                i += 1;
            }
        });
    }
}

impl<'a> Updatable<&Metric> for Calls<'a> {
    fn update(&mut self, metric: &Metric) {
        self.stats = metric.command.stats.iter().map(|it| it.clone()).collect();
        self.sum_calls = metric.command.sum_calls;
        self.sum_usec = metric.command.sum_usec;
        self.sum_usec_per_call = metric.command.sum_usec_per_call;
    }
}

