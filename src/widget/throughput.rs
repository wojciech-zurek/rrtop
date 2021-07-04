use crate::colorscheme::ColorScheme;
use tui::widgets::{Widget, Block, Borders, Paragraph};
use tui::layout::{Rect, Layout, Direction, Constraint};
use tui::buffer::Buffer;
use crate::event::Message;
use crate::update::Updatable;
use tui::style::{Modifier, Style, Color};
use tui::text::Span;
use tui::text::Spans;
use crate::widget::sparkline::{Sparkline, RenderDirection};
use std::collections::VecDeque;
use crate::widget::{title, title_span};

pub struct Throughput<'a> {
    title: String,
    ops_per_sec: VecDeque<u64>,
    total_commands: u128,
    color_scheme: &'a ColorScheme,
    max_elements: usize,
}

impl<'a> Throughput<'a> {
    pub fn new(color_scheme: &'a ColorScheme) -> Self {
        let max_elements = 250;
        Throughput {
            title: "throughput".to_owned(),
            ops_per_sec: VecDeque::with_capacity(max_elements),
            total_commands: 0,
            color_scheme,
            max_elements,
        }
    }
}

impl<'a> Widget for &Throughput<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::default()
            .borders(Borders::ALL)
            .border_style(self.color_scheme.throughput_border)
            .title(title_span(&self.title, self.color_scheme.throughput_title, self.color_scheme.throughput_border))
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
            Spans::from(Span::styled(format!("Total commands: {}", self.total_commands), self.color_scheme.throughput_total_commands_text)),
            Spans::from(Span::styled(format!("         Ops/s: {} ops/s", self.ops_per_sec.front().unwrap_or(&0).to_owned()), self.color_scheme.throughput_ops_text))
        ];
        Paragraph::new(spans).render(chunks[1], buf);

        Sparkline::default()
            .data(self.ops_per_sec.iter().map(|it| *it).collect::<Vec<u64>>().as_slice())
            .show_baseline(true)
            .fill_baseline(true)
            .direction(RenderDirection::RightToLeft)
            .style(self.color_scheme.throughput_sparkline)
            .baseline_style(self.color_scheme.throughput_sparkline_baseline)
            .render(chunks[2], buf);
    }
}

impl<'a> Updatable<&Message> for Throughput<'a> {
    fn update(&mut self, message: &Message) {
        self.total_commands = if let Some(total_commands) = message.info.0.get("total_commands_processed") {
            total_commands.parse::<u128>().unwrap_or(0)
        } else {
            0
        };

        if self.ops_per_sec.len() >= self.max_elements {
            self.ops_per_sec.pop_back();
        }

        let ops_per_sec = if let Some(ops_per_sec) = message.info.0.get("instantaneous_ops_per_sec") {
            ops_per_sec.parse::<u64>().unwrap_or(0)
        } else {
            0
        };

        self.ops_per_sec.push_front(ops_per_sec);
    }
}