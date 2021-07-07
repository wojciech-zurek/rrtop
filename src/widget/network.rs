use crate::colorscheme::theme::Theme;
use tui::widgets::{Widget, Block, Borders, Paragraph};
use tui::layout::{Rect, Layout, Direction, Constraint};
use tui::buffer::Buffer;
use crate::update::Updatable;
use tui::style::{Modifier, Style, Color};
use size::Size;
use tui::text::Span;
use tui::text::Spans;
use crate::widget::sparkline::{Sparkline, RenderDirection};
use std::collections::VecDeque;
use crate::widget::{title, title_span};
use crate::metric::Metric;

pub struct Network<'a> {
    title: String,
    input: VecDeque<u64>,
    output: VecDeque<u64>,
    total_input: i64,
    total_output: i64,
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
            Spans::from(Span::styled(format!("    Rx/s: {}/s", Size::Bytes(self.input.front().unwrap_or(&0).to_owned())), self.theme.network_rx_s_text))
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
            Spans::from(Span::styled(format!("    Tx/s: {}/s", Size::Bytes(self.output.front().unwrap_or(&0).to_owned())), self.theme.network_tx_s_text))
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
        self.total_input = metric.network.total_net_input_bytes;
        self.total_output = metric.network.total_net_output_bytes;

        if self.input.len() >= self.max_elements {
            self.input.pop_back();
        }
        self.input.push_front(metric.network.instantaneous_input_kbps);

        if self.output.len() >= self.max_elements {
            self.output.pop_back();
        }
        self.output.push_front(metric.network.instantaneous_output_kbps);
    }
}