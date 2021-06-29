use crate::colorscheme::ColorScheme;
use tui::widgets::{Widget, Block, Borders, Sparkline, Paragraph};
use tui::layout::{Rect, Layout, Direction, Constraint};
use tui::buffer::Buffer;
use crate::event::Message;
use crate::update::Updatable;
use tui::style::{Modifier, Style, Color};
use size::Size;
use tui::text::Span;
use tui::text::Spans;

pub struct Network<'a> {
    title: String,
    input: Vec<u64>,
    output: Vec<u64>,
    total_input: i64,
    total_output: i64,
    color_scheme: &'a ColorScheme,
}

impl<'a> Network<'a> {
    pub fn new(color_scheme: &'a ColorScheme) -> Self {
        Network {
            title: "Network usage".to_string(),
            input: Vec::new(),
            output: Vec::new(),
            total_input: 0,
            total_output: 0,
            color_scheme,
        }
    }
}

impl<'a> Widget for &Network<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::default()
            .borders(Borders::ALL)
            //.border_style(colorscheme.borders)
            .title(Span::from(self.title.as_str())).render(area, buf);


        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(2),
                    Constraint::Length(2),
                    Constraint::Length(2),
                    Constraint::Length(2),
                ]
                    .as_ref(),
            )
            .horizontal_margin(2)
            .vertical_margin(2)
            .split(area);
        //rx
        let spans = vec![
            Spans::from(Span::from(format!("Total rx: {}", Size::Bytes(self.total_input)))),
            Spans::from(Span::from(format!("Rx/s: {}/s", Size::Bytes(self.input.last().unwrap_or(&0).to_owned()))))
        ];
        Paragraph::new(spans).render(chunks[0], buf);
        Sparkline::default()
            .data(&self.input)
            .style(Style::default().fg(Color::Yellow))
            .render(chunks[1], buf);

        //tx
        let spans = vec![
            Spans::from(Span::from(format!("Total tx: {}", Size::Bytes(self.total_output)))),
            Spans::from(Span::from(format!("Tx/s: {}/s", Size::Bytes(self.output.last().unwrap_or(&0).to_owned()))))
        ];
        Paragraph::new(spans).render(chunks[2], buf);
        Sparkline::default()
            .data(&self.output)
            .style(Style::default().fg(Color::Blue))
            .render(chunks[3], buf);
    }
}

impl<'a> Updatable<&Message> for Network<'a> {
    fn update(&mut self, message: &Message) {
        if let Some(total_input) = message.info.0.get("total_net_input_bytes") {
            self.total_input = total_input.parse::<i64>().unwrap_or(0);
        }

        if let Some(total_output) = message.info.0.get("total_net_output_bytes") {
            self.total_output = total_output.parse::<i64>().unwrap_or(0);
        }

        if let Some(input) = message.info.0.get("instantaneous_input_kbps") {
            let i = input.parse::<f64>().unwrap_or(0.0);
            self.input.push((i * 1000.0) as u64);
        }

        if let Some(output) = message.info.0.get("instantaneous_output_kbps") {
            let o = output.parse::<f64>().unwrap_or(0.0);
            self.output.push((o * 1000.0) as u64);
        }
    }
}