use std::collections::VecDeque;
use crate::colorscheme::ColorScheme;
use crate::update::Updatable;
use crate::event::Message;
use tui::layout::{Rect, Layout, Direction, Constraint};
use tui::buffer::Buffer;
use tui::widgets::{Widget, Dataset, GraphType, Chart, Axis, Block, Borders};
use tui::symbols::Marker;
use tui::text::Span;
use crate::widget::{title, title_span};

pub struct Cpu<'a> {
    title: String,
    cpu_sys: VecDeque<(f64, f64)>,
    cpu_user: VecDeque<(f64, f64)>,
    last_cpu_sys: f64,
    last_cpu_user: f64,
    last_diff_cpu_sys: f64,
    last_diff_cpu_user: f64,
    color_scheme: &'a ColorScheme,
    max_elements: usize,
    update_count: u64,
    tick_rate: f64,
}

impl<'a> Cpu<'a> {
    pub fn new(color_scheme: &'a ColorScheme, tick_rate: u64) -> Self {
        let max_elements = 250;
        Cpu {
            title: "CPU usage".to_owned(),
            cpu_sys: VecDeque::with_capacity(max_elements),
            cpu_user: VecDeque::with_capacity(max_elements),
            last_cpu_sys: 0.0,
            last_cpu_user: 0.0,
            last_diff_cpu_sys: 0.0,
            last_diff_cpu_user: 0.0,
            color_scheme,
            max_elements,
            update_count: 0,
            tick_rate: tick_rate as f64,
        }
    }
}

impl<'a> Widget for &Cpu<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::default()
            .borders(Borders::ALL)
            .border_style(self.color_scheme.cpu_border)
            .title(title_span(&self.title, self.color_scheme.cpu_title, self.color_scheme.cpu_border))
            .render(area, buf);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(6),
                    Constraint::Length(6),
                ]
                    .as_ref(),
            )
            .horizontal_margin(1)
            .vertical_margin(0)
            .split(area);


        //cpu sys
        let cpu_sys = self.cpu_sys.iter().map(|it| (it.0, it.1)).collect::<Vec<(f64, f64)>>();

        let dataset = Dataset::default()
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .style(self.color_scheme.cpu_sys_cpu_dataset)
            .data(&cpu_sys);


        //chart
        Chart::new(vec![dataset])
            .y_axis(Axis::default().bounds([0.0, 15.0]))
            .x_axis(Axis::default()
                .bounds([self.update_count as f64 - area.width as f64, self.update_count as f64 + 1.0])
            )
            .style(self.color_scheme.cpu_chart)
            .render(chunks[0], buf);

        //cpu user
        let cpu_user = self.cpu_user.iter().map(|it| (it.0, it.1 * -1.0)).collect::<Vec<(f64, f64)>>();

        let dataset = Dataset::default()
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .style(self.color_scheme.cpu_user_cpu_dataset)
            .data(&cpu_user);

        //chart
        Chart::new(vec![dataset])
            .y_axis(Axis::default().bounds([-15.0, 0.0]))
            .x_axis(Axis::default()
                .bounds([self.update_count as f64 - area.width as f64, self.update_count as f64 + 1.0])
            )
            .style(self.color_scheme.cpu_chart)
            .render(chunks[1], buf);


        buf.set_string(
            area.x + 3,
            area.y + 2,
            format!(" Sys CPU: {:.02}%", self.last_diff_cpu_sys),
            self.color_scheme.cpu_sys_cpu_text,
        );

        buf.set_string(
            area.x + 3,
            area.y + 9,
            format!("User CPU: {:.02}%", self.last_diff_cpu_user),
            self.color_scheme.cpu_user_cpu_text,
        );
    }
}

impl<'a> Updatable<&Message> for Cpu<'a> {
    fn update(&mut self, message: &Message) {
        self.update_count += 1;

        //cpu sys
        let diff_cpu_sys = if let Some(used_cpu_sys) = message.info.0.get("used_cpu_sys") {
            let cpu_sys = used_cpu_sys.parse::<f64>().unwrap_or(0.0);

            if self.last_cpu_sys == 0.0 {
                self.last_cpu_sys = cpu_sys;
                0.0001
            } else {
                let diff = (cpu_sys - self.last_cpu_sys) / self.tick_rate;
                self.last_cpu_sys = cpu_sys;
                if diff <= 0.0001 {
                    0.0001
                } else {
                    diff
                }
            }
        } else {
            0.0
        };

        self.last_diff_cpu_sys = diff_cpu_sys * 100.0;

        if self.cpu_sys.len() >= self.max_elements {
            self.cpu_sys.pop_back();
        }

        self.cpu_sys.push_front((self.update_count as f64, (diff_cpu_sys * 10_000.0).log2()));

        // cpu user
        let diff_cpu_user = if let Some(used_cpu_user) = message.info.0.get("used_cpu_user") {
            let cpu_user = used_cpu_user.parse::<f64>().unwrap_or(0.0);
            if self.last_cpu_user == 0.0 {
                self.last_cpu_user = cpu_user;
                0.0001
            } else {
                let diff = (cpu_user - self.last_cpu_user) / self.tick_rate;
                self.last_cpu_user = cpu_user;
                if diff <= 0.0001 {
                    0.0001
                } else {
                    diff
                }
            }
        } else {
            0.0
        };

        self.last_diff_cpu_user = diff_cpu_user * 100.0;

        if self.cpu_user.len() >= self.max_elements {
            self.cpu_user.pop_back();
        }

        self.cpu_user.push_front((self.update_count as f64, (diff_cpu_user * 10_000.0).log2()));
    }
}