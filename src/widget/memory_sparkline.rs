use std::collections::VecDeque;
use crate::colorscheme::theme::Theme;
use crate::update::Updatable;
use crate::event::Message;
use tui::layout::{Rect, Layout, Direction, Constraint};
use tui::buffer::Buffer;
use tui::widgets::{Widget, Dataset, GraphType, Chart, Axis, Block, Borders, Paragraph};
use tui::symbols::Marker;
use tui::style::{Style, Color, Modifier};
use tui::text::Span;
use tui::text::Spans;
use size::Size;
use crate::widget::sparkline::{RenderDirection, Sparkline};
use crate::widget::title;

pub struct Memory<'a> {
    title: String,
    used_memory: VecDeque<u64>,
    last_used_memory: u64,
    last_max_memory: u64,
    color_scheme: &'a Theme,
    max_elements: usize,
    update_count: u64,
    tick_rate: f64,
}

impl<'a> Memory<'a> {
    pub fn new(color_scheme: &'a Theme, tick_rate: u64) -> Self {
        let max_elements = 250;
        Memory {
            title: title("Memory usage"),
            used_memory: VecDeque::new(),
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
        Block::default()
            .borders(Borders::ALL)
            //.border_style(colorscheme.borders)
            .title(Span::from(self.title.as_str())).render(area, buf);


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
            Spans::from(Span::styled(format!(" Max memory: {}", Size::Bytes(self.last_max_memory)), Style::default().add_modifier(Modifier::BOLD))),
            Spans::from(Span::styled(format!("Used memory: {}", Size::Bytes(self.last_used_memory)), Style::default().add_modifier(Modifier::BOLD)))
        ];
        Paragraph::new(spans).render(chunks[1], buf);
        Sparkline::default()
            .data(self.used_memory.iter().map(|it| *it).collect::<Vec<u64>>().as_slice())
            .show_baseline(true)
            .fill_baseline(true)
            .direction(RenderDirection::RightToLeft)
            .style(Style::default().fg(Color::Red).bg(Color::Reset))
            .render(chunks[2], buf);
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

        self.used_memory.push_front(used_memory);

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
    }
}