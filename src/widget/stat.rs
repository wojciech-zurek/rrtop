use std::collections::VecDeque;
use crate::colorscheme::theme::Theme;
use crate::update::Updatable;
use crate::event::Message;
use tui::widgets::{StatefulWidget, TableState, Cell, Row, Table, Block, Borders};
use tui::buffer::{Buffer};
use tui::layout::{Rect, Constraint};
use tui::text::Span;
use size::Size;
use crate::widget::title_span;
use std::time::Instant;
use chrono::{Duration, Local, DateTime, Utc};

pub struct Stat<'a> {
    title: String,
    headers: Vec<String>,
    time_slices: VecDeque<TimeSlice>,
    theme: &'a Theme,
    max_elements: usize,
    tick_rate: f64,
}

impl<'a> Stat<'a> {
    pub fn new(theme: &'a Theme, tick_rate: u64) -> Self {
        let max_elements = 50;
        Stat {
            title: "stat".to_owned(),
            headers: vec![" time".to_owned(), "ops".to_owned(), "user".to_owned(), "sys".to_owned(), "memUsed".to_owned(), "memRss".to_owned(), "fRatio".to_owned()],
            time_slices: VecDeque::with_capacity(max_elements),
            theme,
            max_elements,
            tick_rate: tick_rate as f64,
        }
    }
}

struct TimeSlice {
    time: chrono::DateTime<Local>,
    cpu_user_time: f64,
    cpu_sys_time: f64,
    used_memory: u64,
    used_rss_memory: u64,
    memory_fragmentation_ratio: f32,
    ops_per_sec: u64,
}

impl TimeSlice {
    fn new(cpu_user_time: f64, cpu_sys_time: f64, used_memory: u64, used_rss_memory: u64, memory_fragmentation_ratio: f32, ops_per_sec: u64) -> Self {
        TimeSlice {
            time: Local::now(),
            cpu_user_time,
            cpu_sys_time,
            used_memory,
            used_rss_memory,
            memory_fragmentation_ratio,
            ops_per_sec,
        }
    }
}

impl<'a> StatefulWidget for &Stat<'a> {
    type State = TableState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let header_cells = self.headers
            .iter()
            .map(|h| Cell::from(h.to_owned()).style(self.theme.stat_table_header));
        let header = Row::new(header_cells)
            .height(1)
            .bottom_margin(0);

        let rows = self.time_slices.iter().enumerate().map(|it| {
            let style1 = Theme::color_table_cell(self.theme.stat_table_row_top_1, self.theme.stat_table_row_bottom, it.0 as u8, area.height - 1);
            let style2 = Theme::color_table_cell(self.theme.stat_table_row_top_2, self.theme.stat_table_row_bottom, it.0 as u8, area.height - 1);

            vec![
                Cell::from(Span::styled(format!("{}", it.1.time.format(" %H:%M:%S")), style1)),
                Cell::from(Span::styled(format!("{}", it.1.ops_per_sec), style2)),
                Cell::from(Span::styled(format!("{}", it.1.cpu_user_time), style2)),
                Cell::from(Span::styled(format!("{}", it.1.cpu_sys_time), style2)),
                Cell::from(Span::styled(format!("{}", Size::Bytes(it.1.used_memory)), style1)),
                Cell::from(Span::styled(format!("{}", Size::Bytes(it.1.used_rss_memory)), style1)),
                Cell::from(Span::styled(format!("{}", it.1.memory_fragmentation_ratio), style2)),
            ]
        }).map(|it| Row::new(it)).collect::<Vec<Row>>();

        Table::new(rows)
            .header(header)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(self.theme.stat_border)
                .title(title_span(&self.title, self.theme.stat_title, self.theme.stat_border))
            )
            .widths(&[
                Constraint::Ratio(3, 20),
                Constraint::Ratio(2, 20),
                Constraint::Ratio(3, 20),
                Constraint::Ratio(3, 20),
                Constraint::Ratio(3, 20),
                Constraint::Ratio(3, 20),
                Constraint::Ratio(3, 20),
            ]).render(area, buf, state);
    }
}

impl<'a> Updatable<&Message> for Stat<'a> {
    fn update(&mut self, message: &Message) {
        if self.time_slices.len() >= self.max_elements {
            self.time_slices.pop_back();
        }

        let cpu_user_time = if let Some(cpu_user_time) = message.info.0.get("used_cpu_user") {
            cpu_user_time.parse::<f64>().unwrap_or(0.0)
        } else {
            0.0
        };

        let cpu_sys_time = if let Some(cpu_sys_time) = message.info.0.get("used_cpu_sys") {
            cpu_sys_time.parse::<f64>().unwrap_or(0.0)
        } else {
            0.0
        };

        let used_memory = if let Some(used_memory) = message.info.0.get("used_memory") {
            used_memory.parse::<u64>().unwrap_or(0)
        } else {
            0
        };

        let used_rss_memory = if let Some(used_rss_memory) = message.info.0.get("used_memory_rss") {
            used_rss_memory.parse::<u64>().unwrap_or(0)
        } else {
            0
        };

        let memory_fragmentation_ratio = if let Some(memory_fragmentation_ratio) = message.info.0.get("mem_fragmentation_ratio") {
            memory_fragmentation_ratio.parse::<f32>().unwrap_or(0.0)
        } else {
            0.0
        };

        let ops_per_sec = if let Some(ops_per_sec) = message.info.0.get("instantaneous_ops_per_sec") {
            ops_per_sec.parse::<u64>().unwrap_or(0)
        } else {
            0
        };

        self.time_slices.push_front(TimeSlice::new(
            cpu_user_time,
            cpu_sys_time,
            used_memory,
            used_rss_memory,
            memory_fragmentation_ratio,
            ops_per_sec));
    }
}