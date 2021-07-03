use std::collections::VecDeque;
use crate::colorscheme::ColorScheme;
use crate::update::Updatable;
use crate::event::Message;
use tui::layout::{Rect, Layout, Direction, Constraint};
use tui::buffer::Buffer;
use tui::widgets::{Widget, Dataset, GraphType, Chart, Axis, Block, Borders};
use tui::symbols::Marker;
use tui::style::{Style, Color, Modifier};
use tui::text::Span;
use size::Size;
use crate::widget::{title, title_span};

pub struct Memory<'a> {
    title: String,
    used_memory: VecDeque<(f64, f64)>,
    max_memory: VecDeque<(f64, f64)>,
    last_used_memory: u64,
    last_max_memory: u64,
    color_scheme: &'a ColorScheme,
    max_elements: usize,
    update_count: u64,
    tick_rate: f64,
}

impl<'a> Memory<'a> {
    pub fn new(color_scheme: &'a ColorScheme, tick_rate: u64) -> Self {
        let max_elements = 250;
        Memory {
            title: "memory".to_owned(),
            used_memory: VecDeque::new(),
            max_memory: VecDeque::new(),
            last_used_memory: 0,
            last_max_memory: 0,
            color_scheme,
            max_elements,
            update_count: 0,
            tick_rate: tick_rate as f64,
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
            .style(self.color_scheme.memory_max_memory_dataset)
            .data(&max_memory);

        //used memory
        let used_memory = self.used_memory.iter().map(|it| (it.0, it.1)).collect::<Vec<(f64, f64)>>();

        let used_memory_dataset = Dataset::default()
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .style(self.color_scheme.memory_used_memory_dataset)
            .data(&used_memory);

        //chart
        Chart::new(vec![max_memory_dataset, used_memory_dataset])
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(self.color_scheme.memory_border)
                .title(title_span(&self.title, self.color_scheme.memory_title, self.color_scheme.memory_border))
            )
            .style(self.color_scheme.memory_chart)
            .y_axis(Axis::default().bounds([24.2, (self.last_max_memory as f64).log2()]))
            .x_axis(Axis::default()
                .bounds([self.update_count as f64 - area.width as f64, self.update_count as f64 + 1.0])
            )
            .render(area, buf);

        buf.set_string(
            area.x + 3,
            area.y + 2,
            format!(" Max memory: {}", Size::Bytes(self.last_max_memory)),
            self.color_scheme.memory_max_memory_text,
        );

        buf.set_string(
            area.x + 3,
            area.y + 9,
            format!("Used memory: {}", Size::Bytes(self.last_used_memory)),
            self.color_scheme.memory_used_memory_text,
        );
    }
}

impl<'a> Updatable<&Message> for Memory<'a> {
    fn update(&mut self, message: &Message) {
        self.update_count += 1;

        //used memory
        let used_memory = if let Some(used_memory) = message.info.0.get("used_memory") {
            used_memory.parse::<u64>().unwrap_or(0)
        } else {
            0
        };

        self.last_used_memory = used_memory;

        if self.used_memory.len() >= self.max_elements {
            self.used_memory.pop_back();
        }

        self.used_memory.push_front((self.update_count as f64, (used_memory as f64).log2()));

        // max memory
        let max_memory = if let Some(max_memory) = message.info.0.get("maxmemory") {
            let max_memory = max_memory.parse::<u64>().unwrap_or(0);
            if max_memory == 0 {
                if let Some(max_memory) = message.info.0.get("total_system_memory") {
                    max_memory.parse::<u64>().unwrap_or(0)
                } else {
                    0
                }
            } else {
                max_memory
            }
        } else {
            0
        };

        self.last_max_memory = max_memory;

        if self.max_memory.len() >= self.max_elements {
            self.max_memory.pop_back();
        }
        self.max_memory.push_front((self.update_count as f64, (max_memory as f64).log2()));
    }
}