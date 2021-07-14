use std::collections::VecDeque;

use size::Size;
use tui::buffer::Buffer;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::text::Span;
use tui::text::Spans;
use tui::widgets::{Block, Borders, Paragraph, Widget};

use crate::colorscheme::theme::Theme;
use crate::metric::Metric;
use crate::update::Updatable;
use crate::widget::title_span;
use crate::widget::sparkline::{RenderDirection, Sparkline};

pub struct Memory<'a> {
    title: String,
    used_memory: VecDeque<u64>,
    rss_memory: VecDeque<u64>,
    last_used_memory: u64,
    last_rss_memory: u64,
    last_max_memory: u64,
    last_fragmentation_ratio: f32,
    theme: &'a Theme,
    max_elements: usize,
}

impl<'a> Memory<'a> {
    pub fn new(theme: &'a Theme) -> Self {
        let max_elements = 250;
        Memory {
            title: "memory".to_owned(),
            used_memory: VecDeque::new(),
            rss_memory: VecDeque::new(),
            last_used_memory: 0,
            last_rss_memory: 0,
            last_max_memory: 0,
            last_fragmentation_ratio: 0.0,
            theme,
            max_elements,
        }
    }
}

impl<'a> Widget for &Memory<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::default()
            .borders(Borders::ALL)
            .border_style(self.theme.memory_border)
            .title(title_span(&self.title, self.theme.network_title, self.theme.network_border))
            .render(area, buf);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Length(2),
                    Constraint::Length(2),
                    Constraint::Length(1),
                    Constraint::Length(2),
                    Constraint::Length(2),
                ]
                    .as_ref(),
            )
            .horizontal_margin(2)
            .vertical_margin(1)
            .split(area);
        //used memory
        let spans = vec![
            Spans::from(Span::styled(format!(" Max memory: {}", Size::Bytes(self.last_max_memory)), self.theme.memory_used_memory_text)),
            Spans::from(Span::styled(format!("Used memory: {}", Size::Bytes(self.last_used_memory)), self.theme.memory_used_memory_text))
        ];
        Paragraph::new(spans).render(chunks[1], buf);
        Sparkline::default()
            .data(self.used_memory.iter().map(|it| *it).collect::<Vec<u64>>().as_slice())
            .show_baseline(true)
            .fill_baseline(true)
            .direction(RenderDirection::RightToLeft)
            .style(self.theme.memory_used_memory_dataset)
            .baseline_style(self.theme.network_rx_sparkline_baseline)
            .render(chunks[2], buf);

        //rss memory
        let spans = vec![
            Spans::from(Span::styled(format!("Frag ratio: {}", self.last_fragmentation_ratio), self.theme.memory_rss_memory_text)),
            Spans::from(Span::styled(format!("RSS memory: {}", Size::Bytes(self.last_rss_memory)), self.theme.memory_rss_memory_text))
        ];
        Paragraph::new(spans).render(chunks[4], buf);
        Sparkline::default()
            .data(self.rss_memory.iter().map(|it| *it).collect::<Vec<u64>>().as_slice())
            .show_baseline(true)
            .fill_baseline(true)
            .direction(RenderDirection::RightToLeft)
            .style(self.theme.memory_rss_memory_dataset)
            .baseline_style(self.theme.network_rx_sparkline_baseline)
            .render(chunks[5], buf);
    }
}

impl<'a> Updatable<&Metric> for Memory<'a> {
    fn update(&mut self, metric: &Metric) {

        //used memory
        self.last_used_memory = metric.memory.used_memory;
        self.last_rss_memory = metric.memory.used_memory_rss;
        self.last_fragmentation_ratio = metric.memory.mem_fragmentation_ratio;

        if self.used_memory.len() >= self.max_elements {
            self.used_memory.pop_back();
        }

        self.used_memory.push_front(metric.memory.used_memory);

        if self.rss_memory.len() >= self.max_elements {
            self.rss_memory.pop_back();
        }

        self.rss_memory.push_front(metric.memory.used_memory_rss);

        self.last_max_memory = metric.memory.max_memory;
    }
}