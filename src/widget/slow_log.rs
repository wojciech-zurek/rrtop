use chrono::{Duration, Local, TimeZone};
use tui::buffer::Buffer;
use tui::layout::{Constraint, Rect};
use tui::text::Span;
use tui::widgets::{Block, Borders, Cell, Row, StatefulWidget, Table, TableState, Widget};

use crate::colorscheme::theme::Theme;
use crate::metric::slow_log;
use crate::metric::slow_log::Log;
use crate::update::Updatable;
use crate::widget::navigation::Navigation;
use crate::widget::title_span;

pub struct SlowLog<'a> {
    title: String,
    headers: Vec<String>,
    theme: &'a Theme,
    logs: Vec<Log>,
    state: TableState,
}

impl<'a> SlowLog<'a> {
    pub fn new(theme: &'a Theme) -> Self {
        SlowLog {
            title: "slow log".to_owned(),
            headers: vec![" id".to_owned(),
                          "time".to_owned(),
                          "exec time".to_owned(),
                          "command".to_owned(),
                          "client ip".to_owned(),
                          "client name".to_owned(),
            ],
            theme,
            logs: vec![],
            state: TableState::default(),
        }
    }
}

impl<'a> Widget for &mut SlowLog<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let header_cells = self.headers
            .iter()
            .map(|h| Cell::from(h.to_owned()).style(self.theme.stat_table_header));
        let header = Row::new(header_cells)
            .height(1)
            .bottom_margin(0);

        let rows = self.logs.iter().enumerate().map(|it| {
            let style1 = Theme::color_table_cell(self.theme.stat_table_row_top_1, self.theme.stat_table_row_bottom, it.0 as u8, area.height.wrapping_sub(1));
            let style2 = Theme::color_table_cell(self.theme.stat_table_row_top_2, self.theme.stat_table_row_bottom, it.0 as u8, area.height.wrapping_sub(1));

            let local = Local.timestamp(it.1.timestamp, 0);
            let duration = Duration::microseconds(it.1.exec_time);

            vec![
                Cell::from(Span::styled(format!(" {}", it.1.id), style1)),
                Cell::from(Span::styled(format!("{}", local.format("%Y-%m-%d %H:%M:%S")), style2)),
                Cell::from(Span::styled(format!("{}s {:0.2}ms {}μs", duration.num_seconds(), duration.num_milliseconds() % 1_000,  it.1.exec_time % 1_000), style2)),
                Cell::from(Span::styled(format!("{}", it.1.command), style2)),
                Cell::from(Span::styled(format!("{}", it.1.client_ip), style1)),
                Cell::from(Span::styled(format!("{}", it.1.client_name), style1)),
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
                Constraint::Ratio(1, 6),
                Constraint::Ratio(1, 6),
                Constraint::Ratio(1, 6),
                Constraint::Ratio(1, 6),
                Constraint::Ratio(1, 6),
                Constraint::Ratio(1, 6),
            ]);

        <Table as StatefulWidget>::render(table, area, buf, &mut self.state)
    }
}

impl<'a> Updatable<slow_log::SlowLog> for SlowLog<'a> {
    fn update(&mut self, logs: slow_log::SlowLog) {
        self.logs = logs.logs;
    }
}

impl<'a> Navigation for SlowLog<'a> {
    fn state(&mut self) -> &mut TableState {
        &mut self.state
    }

    fn len(&self) -> usize {
        self.logs.len()
    }
}