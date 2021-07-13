use std::collections::VecDeque;
use crate::colorscheme::theme::Theme;
use crate::update::Updatable;
use tui::layout::{Rect, Layout, Direction, Constraint};
use tui::buffer::Buffer;
use tui::widgets::{Widget, Dataset, GraphType, Chart, Axis, Block, Borders};
use tui::symbols::Marker;
use tui::style::{Style, Color, Modifier};
use tui::text::Span;
use size::Size;
use crate::widget::{title, title_span};
use crate::metric::Metric;

pub struct Memory<'a> {
    title: String,
    used_memory: VecDeque<(f64, f64)>,
    used_rss_memory: VecDeque<(f64, f64)>,
    max_memory: VecDeque<(f64, f64)>,
    last_used_memory: u64,
    last_rss_memory: u64,
    last_max_memory: u64,
    theme: &'a Theme,
    max_elements: usize,
    update_count: u64,
}

impl<'a> Memory<'a> {
    pub fn new(theme: &'a Theme) -> Self {
        let max_elements = 250;
        Memory {
            title: "memory".to_owned(),
            used_memory: VecDeque::new(),
            used_rss_memory: VecDeque::new(),
            max_memory: VecDeque::new(),
            last_used_memory: 0,
            last_rss_memory: 0,
            last_max_memory: 0,
            theme,
            max_elements,
            update_count: 0,
        }
    }
}

impl<'a> Widget for &Memory<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {

        //max memory
        let max_memory = self.max_memory.iter().map(|it| (it.0, it.1)).collect::<Vec<(f64, f64)>>();

        let max_memory_dataset = Dataset::default()
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .style(self.theme.memory_max_memory_dataset)
            .data(&max_memory);

        //used memory
        let used_memory = self.used_memory.iter().map(|it| (it.0, it.1)).collect::<Vec<(f64, f64)>>();

        let used_memory_dataset = Dataset::default()
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .style(self.theme.memory_used_memory_dataset)
            .data(&used_memory);

        //rss memory
        let used_rss_memory = self.used_rss_memory.iter().map(|it| (it.0, it.1)).collect::<Vec<(f64, f64)>>();

        let rss_memory_dataset = Dataset::default()
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .style(self.theme.memory_rss_memory_dataset)
            .data(&used_rss_memory);

        //chart
        Chart::new(vec![max_memory_dataset, used_memory_dataset, rss_memory_dataset])
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(self.theme.memory_border)
                .title(title_span(&self.title, self.theme.memory_title, self.theme.memory_border))
            )
            .style(self.theme.memory_chart)
            .y_axis(Axis::default().bounds([24.2, (self.last_max_memory as f64).log2()]))
            .x_axis(Axis::default()
                .bounds([self.update_count as f64 - area.width as f64, self.update_count as f64 + 1.0])
            )
            .render(area, buf);

        buf.set_string(
            area.x + 3,
            area.y + 1,
            format!(" Max memory: {}", Size::Bytes(self.last_max_memory)),
            self.theme.memory_max_memory_text,
        );

        buf.set_string(
            area.x + 3,
            area.y + 2,
            format!(" RSS memory: {}", Size::Bytes(self.last_rss_memory)),
            self.theme.memory_rss_memory_text,
        );

        buf.set_string(
            area.x + 3,
            area.y + 3,
            format!("Used memory: {}", Size::Bytes(self.last_used_memory)),
            self.theme.memory_used_memory_text,
        );
    }
}

impl<'a> Updatable<&Metric> for Memory<'a> {
    fn update(&mut self, metric: &Metric) {
        self.update_count += 1;

        self.last_used_memory = metric.memory.used_memory;

        if self.used_memory.len() >= self.max_elements {
            self.used_memory.pop_back();
        }

        self.used_memory.push_front((self.update_count as f64, (metric.memory.used_memory as f64).log2()));

        self.last_rss_memory = metric.memory.used_memory_rss;

        if self.used_rss_memory.len() >= self.max_elements {
            self.used_rss_memory.pop_back();
        }

        self.used_rss_memory.push_front((self.update_count as f64, (metric.memory.used_memory_rss as f64).log2()));

        self.last_max_memory = metric.memory.max_memory;

        if self.max_memory.len() >= self.max_elements {
            self.max_memory.pop_back();
        }
        self.max_memory.push_front((self.update_count as f64, (metric.memory.max_memory as f64).log2()));
    }
}