use std::collections::VecDeque;

use chrono::Local;
use size::Size;
use tui::buffer::Buffer;
use tui::layout::{Constraint, Rect};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Cell, Row, StatefulWidget, Table, TableState, Widget};

use crate::colorscheme::theme::Theme;
use crate::metric::Metric;
use crate::update::Updatable;
use crate::widget::formatter::Formatter;
use crate::widget::linegauge::render_line_gauge;
use crate::widget::navigation::Navigation;
use crate::widget::title_span;

pub struct Stat<'a> {
    title: String,
    headers: Vec<String>,
    time_slices: VecDeque<TimeSlice>,
    theme: &'a Theme,
    max_elements: usize,
    state: TableState,
}

impl<'a> Stat<'a> {
    pub fn new(theme: &'a Theme) -> Self {
        let max_elements = 50;
        Stat {
            title: "stat".to_owned(),
            headers: vec![" time".to_owned(),
                          "op/s".to_owned(),
                          "user".to_owned(),
                          "sys".to_owned(),
                          "mem used".to_owned(),
                          "mem rss".to_owned(),
                          "fr ratio".to_owned(),
                          "hit rate".to_owned(),
                          "keys".to_owned(),
                          "exp".to_owned(),
                          "exp/s".to_owned(),
                          "evt/s".to_owned(),
            ],
            time_slices: VecDeque::with_capacity(max_elements),
            theme,
            max_elements,
            state: TableState::default(),
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
    ops_per_sec: f64,
    keys: u64,
    expires: u64,
    hit_rate: f64,
    expired_per_sec: f64,
    evicted_per_sec: f64,
}

impl<'a> Widget for &mut Stat<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let header_cells = self.headers
            .iter()
            .map(|h| Cell::from(h.to_owned()).style(self.theme.stat_table_header));
        let header = Row::new(header_cells)
            .height(1)
            .bottom_margin(0);

        let rows = self.time_slices.iter().enumerate().map(|it| {
            let style1 = Theme::color_table_cell(self.theme.stat_table_row_top_1, self.theme.stat_table_row_bottom, it.0 as u8, area.height.wrapping_sub(1));
            let style2 = Theme::color_table_cell(self.theme.stat_table_row_top_2, self.theme.stat_table_row_bottom, it.0 as u8, area.height.wrapping_sub(1));
            let gauge_style = Theme::color_table_cell(self.theme.stat_table_row_gauge, self.theme.calls_table_row_bottom, it.0 as u8, area.height.wrapping_sub(1));


            vec![
                Cell::from(Span::styled(format!("{}", it.1.time.format(" %H:%M:%S")), style1)),
                Cell::from(Span::styled(format!("{}",  it.1.ops_per_sec.num_format()), style2)),
                Cell::from(Span::styled(format!("{:.02}%", it.1.cpu_user_time), style2)),
                Cell::from(Span::styled(format!("{:.02}%", it.1.cpu_sys_time), style2)),
                Cell::from(Span::styled(format!("{}", Size::Bytes(it.1.used_memory)), style1)),
                Cell::from(Span::styled(format!("{}", Size::Bytes(it.1.used_rss_memory)), style1)),
                Cell::from(Span::styled(format!("{}", it.1.memory_fragmentation_ratio), style2)),
                Cell::from(Spans::from(render_line_gauge(Span::styled(format!("{:>3.2}%", it.1.hit_rate * 100.0), style2), it.1.hit_rate, area.width, style2, gauge_style))),
                Cell::from(Span::styled(format!("{}", it.1.keys.num_format()), style1)),
                Cell::from(Span::styled(format!("{}", it.1.expires), style1)),
                Cell::from(Span::styled(format!("{}", it.1.expired_per_sec), style2)),
                Cell::from(Span::styled(format!("{}", it.1.evicted_per_sec), style2)),
            ]
        }).map(|it| Row::new(it)).collect::<Vec<Row>>();

        let table = Table::new(rows)
            .header(header)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(self.theme.stat_border)
                .title(title_span(&self.title, self.theme.stat_title, self.theme.stat_border))
            )
            .highlight_style(self.theme.stat_table_row_highlight)
            .widths(&[
                Constraint::Ratio(2, 14),//time
                Constraint::Ratio(1, 14),//op/s
                Constraint::Ratio(1, 14),//user
                Constraint::Ratio(1, 14),//sys
                Constraint::Ratio(1, 14),//mem used
                Constraint::Ratio(1, 14),//mem rss
                Constraint::Ratio(1, 14),//frag ratio
                Constraint::Ratio(2, 14),//hit rate
                Constraint::Ratio(1, 14),//keys
                Constraint::Ratio(1, 14),//expires
                Constraint::Ratio(1, 14),//exp/s
                Constraint::Ratio(1, 14),//evt/s
            ]);

        StatefulWidget::render(table, area, buf, &mut self.state);
    }
}

impl<'a> Updatable<&Metric> for Stat<'a> {
    fn update(&mut self, metric: &Metric) {
        if self.time_slices.len() >= self.max_elements {
            self.time_slices.pop_back();
        }

        let cpu_user_time = metric.cpu.last_delta_cpu_user;

        let cpu_sys_time = metric.cpu.last_delta_cpu_sys;

        let used_memory = metric.memory.used_memory;

        let used_rss_memory = metric.memory.used_memory_rss;

        let memory_fragmentation_ratio = metric.memory.mem_fragmentation_ratio;

        let ops_per_sec = metric.stats.last_delta_ops;

        let keys = metric.keyspace.total_keys;
        let expires = metric.keyspace.total_expires;
        let hit_rate = metric.keyspace.keyspace_hit_rate;
        let expired_per_sec = metric.stats.last_delta_expired_keys;
        let evicted_per_sec = metric.stats.last_delta_evicted_keys;

        self.time_slices.push_front(TimeSlice {
            time: Local::now(),
            cpu_user_time,
            cpu_sys_time,
            used_memory,
            used_rss_memory,
            memory_fragmentation_ratio,
            ops_per_sec,
            keys,
            expires,
            hit_rate,
            expired_per_sec,
            evicted_per_sec,
        });
    }
}

impl<'a> Navigation for Stat<'a> {
    fn state(&mut self) -> &mut TableState {
        &mut self.state
    }

    fn len(&self) -> usize {
        self.time_slices.len()
    }
}