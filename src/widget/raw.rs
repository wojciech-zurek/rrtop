use tui::buffer::Buffer;
use tui::layout::{Constraint, Rect};
use tui::text::Span;
use tui::widgets::{Block, Borders, Cell, Row, StatefulWidget, Table, TableState, Widget};

use crate::colorscheme::theme::Theme;
use crate::metric::Metric;
use crate::update::Updatable;
use crate::widget::navigation::Navigation;
use crate::widget::title_span;

pub struct Raw<'a> {
    title: String,
    headers: Vec<String>,
    theme: &'a Theme,
    values: Vec<(String, String)>,
    state: TableState,
}

impl<'a> Raw<'a> {
    pub fn new(theme: &'a Theme) -> Self {
        Raw {
            title: "raw info".to_owned(),
            headers: vec![" key".to_owned(),
                          "value".to_owned(), ],
            theme,
            values: vec![],
            state: TableState::default(),
        }
    }
}

impl<'a> Widget for &mut Raw<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let header_cells = self.headers
            .iter()
            .map(|h| Cell::from(h.to_owned()).style(self.theme.stat_table_header));
        let header = Row::new(header_cells)
            .height(1)
            .bottom_margin(0);

        let rows = self.values.iter().enumerate().map(|it| {
            let style1 = Theme::color_table_cell(self.theme.stat_table_row_top_1, self.theme.stat_table_row_bottom, it.0 as u8, area.height.wrapping_sub(1));
            let style2 = Theme::color_table_cell(self.theme.stat_table_row_top_2, self.theme.stat_table_row_bottom, it.0 as u8, area.height.wrapping_sub(1));

            vec![
                Cell::from(Span::styled(format!(" {}", it.1.0), style1)),
                Cell::from(Span::styled(format!(" {}", it.1.1), style2)),
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
                Constraint::Ratio(1, 2),
                Constraint::Ratio(1, 2),
            ]);

        <Table as StatefulWidget>::render(table, area, buf, &mut self.state)
    }
}

impl<'a> Updatable<&Metric> for Raw<'a> {
    fn update(&mut self, metric: &Metric) {
        let mut values = metric.raw.0
            .iter()
            .map(|it| (it.0.clone(), it.1.clone())).collect::<Vec<(String, String)>>();
        values.sort_by(|a, b| a.cmp(&b));

        self.values = values;
    }
}

impl<'a> Navigation for Raw<'a> {
    fn state(&mut self) -> &mut TableState {
        &mut self.state
    }

    fn len(&self) -> usize {
        self.values.len()
    }
}