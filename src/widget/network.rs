use crate::colorscheme::theme::Theme;
use tui::widgets::{Widget, Block, Borders, Paragraph};
use tui::layout::{Rect, Layout, Direction, Constraint};
use tui::buffer::Buffer;
use crate::event::Message;
use crate::update::Updatable;
use tui::style::{Modifier, Style, Color};
use size::Size;
use tui::text::Span;
use tui::text::Spans;
use crate::widget::sparkline::{Sparkline, RenderDirection};
use std::collections::VecDeque;
use crate::widget::{title, title_span};

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
            theme: theme,
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

impl<'a> Updatable<&Message> for Network<'a> {
    fn update(&mut self, message: &Message) {
        if let Some(total_input) = message.info.0.get("total_net_input_bytes") {
            self.total_input = total_input.parse::<i64>().unwrap_or(0);
        } else {
            self.total_input = 0;
        }

        if let Some(total_output) = message.info.0.get("total_net_output_bytes") {
            self.total_output = total_output.parse::<i64>().unwrap_or(0);
        } else {
            self.total_output = 0;
        }

        if self.input.len() >= self.max_elements {
            self.input.pop_back();
        }
        if let Some(input) = message.info.0.get("instantaneous_input_kbps") {
            let i = input.parse::<f64>().unwrap_or(0.0);
            self.input.push_front((i * 1000.0) as u64);
        } else {
            self.input.push_front(0);
        }

        if self.output.len() >= self.max_elements {
            self.output.pop_back();
        }

        if let Some(output) = message.info.0.get("instantaneous_output_kbps") {
            let o = output.parse::<f64>().unwrap_or(0.0);
            self.output.push_front((o * 1000.0) as u64);
        } else {
            self.output.push_front(0);
        }
    }
}