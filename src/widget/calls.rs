use std::cmp::Ordering;

use tui::buffer::Buffer;
use tui::layout::{Constraint, Rect};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Cell, Row, StatefulWidget, Table, TableState, Widget};

use crate::colorscheme::theme::Theme;
use crate::metric::command::CmdStat;
use crate::metric::Metric;
use crate::update::Updatable;
use crate::widget::linegauge::render_line_gauge;
use crate::widget::navigation::Navigation;
use crate::widget::title_span;

pub struct Calls<'a> {
    title: String,
    theme: &'a Theme,
    headers: Vec<String>,
    stats: Vec<CmdStat>,
    sum_calls: f64,
    sum_usec: f64,
    sum_usec_per_call: f64,
    sort_by: Sort,
    state: TableState,
}

impl<'a> Calls<'a> {
    pub fn new(theme: &'a Theme) -> Self {
        let sort_by = Sort::Calls;
        Calls {
            title: sort_by.title(),
            theme,
            headers: vec![" command".to_owned(),
                          "calls".to_owned(),
                          "usec".to_owned(),
                          "usec per call".to_owned(),
            ],
            stats: Vec::new(),
            sum_calls: 0.0,
            sum_usec: 0.0,
            sum_usec_per_call: 0.0,
            sort_by,
            state: TableState::default(),
        }
    }

    pub fn sort_next(&mut self) {
        let next = match self.sort_by {
            Sort::Calls => { Sort::Usec }
            Sort::Usec => { Sort::UsecPerCall }
            Sort::UsecPerCall => { Sort::Calls }
        };
        self.title = next.title();
        self.sort_by = next;
        self.stats.sort_by(self.sort_by.sort());
    }
}

impl<'a> Widget for &mut Calls<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let header_cells = self.headers
            .iter()
            .map(|h| Cell::from(h.to_owned()).style(self.theme.calls_table_header));
        let header = Row::new(header_cells)
            .height(1)
            .bottom_margin(0);

        let rows = self.stats.iter().enumerate().map(|it| {
            let style1 = Theme::color_table_cell(self.theme.calls_table_row_top_1, self.theme.calls_table_row_bottom, it.0 as u8, area.height.wrapping_sub(1));
            let style2 = Theme::color_table_cell(self.theme.calls_table_row_top_2, self.theme.calls_table_row_bottom, it.0 as u8, area.height.wrapping_sub(1));

            let calls = it.1.calls as f64 / self.sum_calls;
            let usec = it.1.usec as f64 / self.sum_usec;
            let usec_per_call = it.1.usec_per_call as f64 / self.sum_usec_per_call;

            vec![
                Cell::from(Span::styled(format!(" {}", it.1.name), style1)),
                Cell::from(Spans::from(render_line_gauge(it.1.calls, calls, area.width, style2))),
                Cell::from(Spans::from(render_line_gauge(it.1.usec, usec, area.width, style2))),
                Cell::from(Spans::from(render_line_gauge(it.1.usec_per_call, usec_per_call, area.width, style2))),
            ]
        }).map(|it| Row::new(it)).collect::<Vec<Row>>();

        let table = Table::new(rows)
            .header(header)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(self.theme.calls_border)
                .title(title_span(&self.title, self.theme.calls_title, self.theme.calls_border))
            )
            .highlight_style(self.theme.calls_table_row_highlight)
            .widths(&[
                Constraint::Ratio(2, 20),
                Constraint::Ratio(6, 20),
                Constraint::Ratio(6, 20),
                Constraint::Ratio(6, 20),
            ]);

        StatefulWidget::render(table, area, buf, &mut self.state);
    }
}

impl<'a> Updatable<&Metric> for Calls<'a> {
    fn update(&mut self, metric: &Metric) {
        let mut stats = metric.command.stats
            .iter()
            .map(|it| it.clone()).collect::<Vec<CmdStat>>();
        stats.sort_by(self.sort_by.sort());

        self.stats = stats;

        self.sum_calls = metric.command.sum_calls;
        self.sum_usec = metric.command.sum_usec;
        self.sum_usec_per_call = metric.command.sum_usec_per_call;
    }
}

impl<'a> Navigation for Calls<'a> {
    fn state(&mut self) -> &mut TableState {
        &mut self.state
    }

    fn len(&self) -> usize {
        self.stats.len()
    }
}

enum Sort {
    Calls,
    Usec,
    UsecPerCall,
}

impl Sort {
    fn sort(&self) -> fn(a: &CmdStat, b: &CmdStat) -> Ordering {
        match self {
            Sort::Calls => { sort_by_calls }
            Sort::Usec => { sort_by_usec }
            Sort::UsecPerCall => { sort_by_usec_per_call }
        }
    }

    fn title(&self) -> String {
        match self {
            Sort::Calls => "by calls".to_owned(),
            Sort::Usec => "by usec".to_owned(),
            Sort::UsecPerCall => "by usec per call".to_owned()
        }
    }
}

fn sort_by_calls(a: &CmdStat, b: &CmdStat) -> Ordering {
    match a.calls.cmp(&b.calls) {
        Ordering::Equal => {
            a.name.cmp(&b.name)
        }
        Ordering::Less => { Ordering::Greater }
        Ordering::Greater => { Ordering::Less }
    }
}

fn sort_by_usec(a: &CmdStat, b: &CmdStat) -> Ordering {
    match a.usec.cmp(&b.usec) {
        Ordering::Equal => {
            a.name.cmp(&b.name)
        }
        Ordering::Less => { Ordering::Greater }
        Ordering::Greater => { Ordering::Less }
    }
}

fn sort_by_usec_per_call(a: &CmdStat, b: &CmdStat) -> Ordering {
    match a.usec_per_call.partial_cmp(&b.usec_per_call).unwrap_or(Ordering::Equal) {
        Ordering::Equal => {
            a.name.cmp(&b.name)
        }
        Ordering::Less => { Ordering::Greater }
        Ordering::Greater => { Ordering::Less }
    }
}