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
use crate::widget::sparkline::{RenderDirection, Sparkline};
use crate::widget::title_span;

pub struct Network<'a> {
    title: String,
    input: VecDeque<u64>,
    output: VecDeque<u64>,
    total_input: i64,
    total_output: i64,
    last_delta_input: i64,
    last_delta_output: i64,
    theme: &'a Theme,
    max_elements: usize,
}

impl<'a> Network<'a> {
    pub fn new(theme: &'a Theme) -> Self {
        let max_elements = 250;
        Network {
            title: "network".to_owned(),
            input: VecDeque::with_capacity(max_elements),
            output: VecDeque::with_capacity(max_elements),
            total_input: 0,
            total_output: 0,
            last_delta_input: 0,
            last_delta_output: 0,
            theme,
            max_elements,
        }
    }
}

impl<'a> Widget for &Network<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::default()
            .borders(Borders::ALL)
            .border_style(self.theme.network_border)
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
        //rx
        let spans = vec![
            Spans::from(Span::styled(format!("Total rx: {}", Size::Bytes(self.total_input)), self.theme.network_rx_total_text)),
            Spans::from(Span::styled(format!("    Rx/s: {}/s", Size::Bytes(self.last_delta_input)), self.theme.network_rx_s_text))
        ];
        Paragraph::new(spans).render(chunks[1], buf);
        Sparkline::default()
            .data(self.input.iter().map(|it| *it).collect::<Vec<u64>>().as_slice())
            .show_baseline(true)
            .fill_baseline(true)
            .direction(RenderDirection::RightToLeft)
            .style(self.theme.network_rx_sparkline)
            .baseline_style(self.theme.network_rx_sparkline_baseline)
            .render(chunks[2], buf);

        //tx
        let spans = vec![
            Spans::from(Span::styled(format!("Total tx: {}", Size::Bytes(self.total_output)), self.theme.network_tx_total_text)),
            Spans::from(Span::styled(format!("    Tx/s: {}/s", Size::Bytes(self.last_delta_output)), self.theme.network_tx_s_text))
        ];
        Paragraph::new(spans).render(chunks[4], buf);
        Sparkline::default()
            .data(self.output.iter().map(|it| *it).collect::<Vec<u64>>().as_slice())
            .show_baseline(true)
            .fill_baseline(true)
            .direction(RenderDirection::RightToLeft)
            .style(self.theme.network_tx_sparkline)
            .baseline_style(self.theme.network_rx_sparkline_baseline)
            .render(chunks[5], buf);
    }
}

impl<'a> Updatable<&Metric> for Network<'a> {
    fn update(&mut self, metric: &Metric) {
        self.total_input = metric.stats.total_net_input_bytes;
        self.total_output = metric.stats.total_net_output_bytes;
        self.last_delta_input = metric.stats.last_delta_network_input_bps as i64;
        self.last_delta_output = metric.stats.last_delta_network_output_bps as i64;

        if self.input.len() >= self.max_elements {
            self.input.pop_back();
        }
        self.input.push_front(metric.stats.last_delta_network_input_bps);

        if self.output.len() >= self.max_elements {
            self.output.pop_back();
        }
        self.output.push_front(metric.stats.last_delta_network_output_bps);
    }
}