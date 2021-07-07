use std::collections::VecDeque;
use crate::colorscheme::theme::Theme;
use crate::update::Updatable;
use tui::layout::{Rect, Layout, Direction, Constraint};
use tui::buffer::Buffer;
use tui::widgets::{Widget, Dataset, GraphType, Chart, Axis, Block, Borders};
use tui::symbols::Marker;
use tui::text::Span;
use crate::widget::{title, title_span, MIN_DOT_SYMBOL};
use tui::style::Color;
use std::cmp::Ordering;
use crate::metric::Metric;

pub struct Cpu<'a> {
    title: String,
    cpu_sys: VecDeque<(f64, f64)>,
    cpu_user: VecDeque<(f64, f64)>,
    last_delta_cpu_sys: f64,
    last_delta_cpu_user: f64,
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
            last_delta_cpu_sys: 0.0,
            last_delta_cpu_user: 0.0,
            theme,
            max_elements,
            update_count: 0,
            tick_rate: tick_rate as f64,
        }
    }

    fn min_cpu_user(&self, size: usize) -> f64 {
        Cpu::min(&self.cpu_user, size)
    }

    fn min_cpu_sys(&self, size: usize) -> f64 {
        Cpu::min(&self.cpu_sys, size)
    }

    fn max_cpu_user(&self, size: usize) -> f64 {
        Cpu::max(&self.cpu_user, size)
    }

    fn max_cpu_sys(&self, size: usize) -> f64 {
        Cpu::max(&self.cpu_sys, size)
    }

    pub fn min(v: &VecDeque<(f64, f64)>, size: usize) -> f64 {
        v.iter()
            .take(size)
            .map(|it| it.1)
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal)).unwrap_or(0.0)
    }

    pub fn max(v: &VecDeque<(f64, f64)>, size: usize) -> f64 {
        v.iter()
            .take(size)
            .map(|it| it.1)
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal)).unwrap_or(0.0)
    }
}

impl<'a> Widget for &Cpu<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::default()
            .borders(Borders::ALL)
            .border_style(self.theme.cpu_border)
            .title(title_span(&self.title, self.theme.cpu_title, self.theme.cpu_border))
            .render(area, buf);

        let main_chunk = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Ratio(7, 10),
                    Constraint::Ratio(3, 10),
                ]
                    .as_ref(),
            )
            .horizontal_margin(1)
            .vertical_margin(0)
            .split(area);

        //charts
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
            .split(main_chunk[0]);

        for i in main_chunk[0].left() + 2..main_chunk[0].right() - 2 {
            buf.get_mut(i, 5)
                .set_style(self.theme.cpu_chart_line)
                .set_symbol(MIN_DOT_SYMBOL);
        }

        //cpu sys
        let cpu_sys = self.cpu_sys.iter().map(|it| (it.0, it.1.log2())).collect::<Vec<(f64, f64)>>();
        let min_cpu_sys = &self.min_cpu_sys(chunks[1].width as usize);
        let max_cpu_sys = &self.max_cpu_sys(chunks[1].width as usize);

        let dataset = Dataset::default()
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .style(self.theme.cpu_sys_cpu_dataset)
            .data(&cpu_sys);

        //chart
        Chart::new(vec![dataset])
            .y_axis(Axis::default().bounds([min_cpu_sys.log2(), max_cpu_sys.log2()]))
            .x_axis(Axis::default()
                .bounds([self.update_count as f64 - area.width as f64, self.update_count as f64 + 1.0])
            )
            .style(self.theme.cpu_chart)
            .render(chunks[1], buf);

        //cpu user
        let cpu_user = self.cpu_user.iter().map(|it| (it.0, it.1.log2() * -1.0)).collect::<Vec<(f64, f64)>>();
        let min_cpu_user = &self.min_cpu_user(chunks[2].width as usize);
        let max_cpu_user = &self.max_cpu_user(chunks[2].width as usize);

        let dataset = Dataset::default()
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .style(self.theme.cpu_user_cpu_dataset)
            .data(&cpu_user);

        //chart
        Chart::new(vec![dataset])
            .y_axis(Axis::default().bounds([max_cpu_user.log2() * -1.0, min_cpu_user.log2() * -1.0]))
            .x_axis(Axis::default()
                .bounds([self.update_count as f64 - area.width as f64, self.update_count as f64 + 1.0])
            )
            .style(self.theme.cpu_chart)
            .render(chunks[2], buf);

        //inner block
        Block::default()
            .borders(Borders::ALL)
            .border_style(self.theme.cpu_border)
            .title(title_span(&self.title, self.theme.cpu_title, self.theme.cpu_border))
            .render(Rect::new(main_chunk[1].x + 5, main_chunk[1].y + 2, main_chunk[1].width - 5, main_chunk[1].height - 4), buf);

        buf.set_string(
            main_chunk[1].x + 7,
            area.y + 4,
            format!(" Sys CPU: {:.02}%", self.last_delta_cpu_sys),
            self.theme.cpu_sys_cpu_text,
        );

        buf.set_string(
            main_chunk[1].x,
            main_chunk[1].y + 1,
            format!("{:.02}", max_cpu_sys),
            self.theme.cpu_border,
        );

        buf.set_string(
            main_chunk[1].x,
            main_chunk[1].y + 5,
            format!("{:.02}", min_cpu_sys),
            self.theme.cpu_border,
        );

        buf.set_string(
            main_chunk[1].x + 7,
            main_chunk[1].y + 7,
            format!("User CPU: {:.02}%", self.last_delta_cpu_user),
            self.theme.cpu_user_cpu_text,
        );

        buf.set_string(
            main_chunk[1].x,
            main_chunk[1].y + 10,
            format!("{:.02}", max_cpu_user),
            self.theme.cpu_border,
        );

        buf.set_string(
            main_chunk[1].x,
            main_chunk[1].y + 6,
            format!("{:.02}", min_cpu_user),
            self.theme.cpu_border,
        );
    }
}

impl<'a> Updatable<&Metric> for Cpu<'a> {
    fn update(&mut self, metric: &Metric) {
        self.update_count += 1;

        self.last_delta_cpu_sys = metric.cpu.last_delta_cpu_sys;

        if self.cpu_sys.len() >= self.max_elements {
            self.cpu_sys.pop_back();
        }

        self.cpu_sys.push_front((self.update_count as f64, metric.cpu.last_delta_cpu_sys));

        self.last_delta_cpu_user = metric.cpu.last_delta_cpu_user;

        if self.cpu_user.len() >= self.max_elements {
            self.cpu_user.pop_back();
        }

        self.cpu_user.push_front((self.update_count as f64, metric.cpu.last_delta_cpu_user));
    }
}