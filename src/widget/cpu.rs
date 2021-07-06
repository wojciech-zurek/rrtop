use std::collections::VecDeque;
use crate::colorscheme::theme::Theme;
use crate::update::Updatable;
use crate::event::Message;
use tui::layout::{Rect, Layout, Direction, Constraint};
use tui::buffer::Buffer;
use tui::widgets::{Widget, Dataset, GraphType, Chart, Axis, Block, Borders};
use tui::symbols::Marker;
use tui::text::Span;
use crate::widget::{title, title_span, MIN_DOT_SYMBOL};
use tui::style::Color;
use std::cmp::Ordering;

pub struct Cpu<'a> {
    title: String,
    cpu_sys: VecDeque<(f64, f64)>,
    cpu_user: VecDeque<(f64, f64)>,
    last_cpu_sys: f64,
    last_cpu_user: f64,
    last_diff_cpu_sys: f64,
    last_diff_cpu_user: f64,
    theme: &'a Theme,
    max_elements: usize,
    update_count: u64,
    tick_rate: f64,
}

impl<'a> Cpu<'a> {
    pub fn new(theme: &'a Theme, tick_rate: u64) -> Self {
        let max_elements = 250;
        Cpu {
            title: "cpu".to_owned(),
            cpu_sys: VecDeque::with_capacity(max_elements),
            cpu_user: VecDeque::with_capacity(max_elements),
            last_cpu_sys: 0.0,
            last_cpu_user: 0.0,
            last_diff_cpu_sys: 0.0,
            last_diff_cpu_user: 0.0,
            theme,
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
            .border_style(self.theme.cpu_border)
            .title(title_span(&self.title, self.theme.cpu_title, self.theme.cpu_border))
            .render(area, buf);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Length(5),
                    Constraint::Length(5),
                    Constraint::Length(1),
                ]
                    .as_ref(),
            )
            .horizontal_margin(1)
            .vertical_margin(0)
            .split(area);

        for i in area.left() + 2..area.right() - 2 {
            buf.get_mut(i, 5)
                .set_style(self.theme.cpu_chart_line)
                .set_symbol(MIN_DOT_SYMBOL);
        }

        //cpu sys
        let cpu_sys = self.cpu_sys.iter().map(|it| (it.0, it.1.log2())).collect::<Vec<(f64, f64)>>();
        let cpu_sys_min = min(&self.cpu_sys, chunks[1].width as usize);
        let cpu_sys_max = max(&self.cpu_sys, chunks[1].width as usize);

        let dataset = Dataset::default()
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .style(self.theme.cpu_sys_cpu_dataset)
            .data(&cpu_sys);

        //chart
        Chart::new(vec![dataset])
            .y_axis(Axis::default().bounds([cpu_sys_min.log2(), cpu_sys_max.log2()]))
            .x_axis(Axis::default()
                .bounds([self.update_count as f64 - area.width as f64, self.update_count as f64 + 1.0])
            )
            .style(self.theme.cpu_chart)
            .render(chunks[1], buf);

        //cpu user
        let cpu_user = self.cpu_user.iter().map(|it| (it.0, it.1.log2() * -1.0)).collect::<Vec<(f64, f64)>>();

        let cpu_user_min = min(&self.cpu_user, chunks[2].width as usize);
        let cpu_user_max = max(&self.cpu_user, chunks[2].width as usize);

        let dataset = Dataset::default()
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .style(self.theme.cpu_user_cpu_dataset)
            .data(&cpu_user);

        //chart
        Chart::new(vec![dataset])
            .y_axis(Axis::default().bounds([cpu_user_max.log2() * -1.0, cpu_user_min.log2() * -1.0]))
            .x_axis(Axis::default()
                .bounds([self.update_count as f64 - area.width as f64, self.update_count as f64 + 1.0])
            )
            .style(self.theme.cpu_chart)
            .render(chunks[2], buf);


        buf.set_string(
            area.x + 3,
            area.y + 2,
            format!(" Sys CPU: {:.02}%", self.last_diff_cpu_sys),
            self.theme.cpu_sys_cpu_text,
        );

        buf.set_string(
            area.x + 1,
            area.y + 1,
            format!("{:.02}", cpu_sys_max),
            self.theme.cpu_border,
        );

        buf.set_string(
            area.x + 1,
            area.y + 5,
            format!("{:.02}", cpu_sys_min),
            self.theme.cpu_border,
        );

        buf.set_string(
            area.x + 3,
            area.y + 9,
            format!("User CPU: {:.02}%", self.last_diff_cpu_user),
            self.theme.cpu_user_cpu_text,
        );

        buf.set_string(
            area.x + 1,
            area.y + 10,
            format!("{:.02}", cpu_user_max),
            self.theme.cpu_border,
        );

        buf.set_string(
            area.x + 1,
            area.y + 6,
            format!("{:.02}", cpu_user_min),
            self.theme.cpu_border,
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

        self.cpu_sys.push_front((self.update_count as f64, diff_cpu_sys * 100.0));

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

        self.cpu_user.push_front((self.update_count as f64, (diff_cpu_user * 100.0)));
    }
}


fn min(v: &VecDeque<(f64, f64)>, size: usize) -> f64 {
    v.iter()
        .take(size)
        .map(|it| it.1)
        .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal)).unwrap_or(0.0)
}

fn max(v: &VecDeque<(f64, f64)>, size: usize) -> f64 {
    v.iter()
        .take(size)
        .map(|it| it.1)
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal)).unwrap_or(0.0)
}