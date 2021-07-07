use crate::colorscheme::theme::Theme;
use tui::widgets::{Widget, Block, Borders, Paragraph};
use tui::layout::{Rect, Layout, Direction, Constraint};
use tui::buffer::Buffer;
use crate::update::Updatable;
use tui::style::{Modifier, Style, Color};
use tui::text::Span;
use tui::text::Spans;
use crate::widget::sparkline::{Sparkline, RenderDirection};
use std::collections::VecDeque;
use crate::widget::{title, title_span};
use crate::metric::Metric;

pub struct Throughput<'a> {
    title: String,
    ops_per_sec: VecDeque<u64>,
    total_commands: u64,
    theme: &'a Theme,
    max_elements: usize,
}

impl<'a> Throughput<'a> {
    pub fn new(theme: &'a Theme) -> Self {
        let max_elements = 250;
        Throughput {
            title: "throughput".to_owned(),
            ops_per_sec: VecDeque::with_capacity(max_elements),
            total_commands: 0,
            theme,
            max_elements,
        }
    }
}

impl<'a> Widget for &Throughput<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::default()
            .borders(Borders::ALL)
            .border_style(self.theme.throughput_border)
            .title(title_span(&self.title, self.theme.throughput_title, self.theme.throughput_border))
            .render(area, buf);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Length(2),
                    Constraint::Length(2)
                ]
                    .as_ref(),
            )
            .horizontal_margin(2)
            .vertical_margin(1)
            .split(area);

        let spans = vec![
            Spans::from(Span::styled(format!("Total commands: {}", self.total_commands), self.theme.throughput_total_commands_text)),
            Spans::from(Span::styled(format!("         Ops/s: {} ops/s", self.ops_per_sec.front().unwrap_or(&0).to_owned()), self.theme.throughput_ops_text))
        ];
        Paragraph::new(spans).render(chunks[1], buf);

        Sparkline::default()
            .data(self.ops_per_sec.iter().map(|it| *it).collect::<Vec<u64>>().as_slice())
            .show_baseline(true)
            .fill_baseline(true)
            .direction(RenderDirection::RightToLeft)
            .style(self.theme.throughput_sparkline)
            .baseline_style(self.theme.throughput_sparkline_baseline)
            .render(chunks[2], buf);
    }
}

impl<'a> Updatable<&Metric> for Throughput<'a> {
    fn update(&mut self, metric: &Metric) {
        self.total_commands = metric.throughput.total_commands_processed;

        if self.ops_per_sec.len() >= self.max_elements {
            self.ops_per_sec.pop_back();
        }

        self.ops_per_sec.push_front(metric.throughput.instantaneous_ops_per_sec);
    }
}