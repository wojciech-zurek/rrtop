use std::cmp::Ordering;
use std::collections::VecDeque;

use tui::buffer::Buffer;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::text::Span;
use tui::text::Spans;
use tui::widgets::{Block, Borders, Paragraph, Widget};

use crate::colorscheme::theme::Theme;
use crate::metric::Metric;
use crate::update::Updatable;
use crate::widget::{MIN_DOT_SYMBOL, title_span};
use crate::widget::cpu_sys::CpuSys;
use crate::widget::cpu_user::CpuUser;
use crate::widget::throughput::Throughput;

pub struct Cpu<'a> {
    title: String,
    cpu_sys: CpuSys<'a>,
    cpu_user: CpuUser<'a>,
    throughput: Throughput<'a>,
    last_delta_cpu_sys: f64,
    last_delta_cpu_user: f64,
    theme: &'a Theme,
}

impl<'a> Cpu<'a> {
    pub fn new(theme: &'a Theme) -> Self {
        let max_elements = 250;
        Cpu {
            title: "cpu".to_owned(),
            cpu_sys: CpuSys::new(theme, max_elements),
            cpu_user: CpuUser::new(theme, max_elements),
            throughput: Throughput::new(theme),
            last_delta_cpu_sys: 0.0,
            last_delta_cpu_user: 0.0,
            theme,
        }
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
                    Constraint::Ratio(6, 10),
                    Constraint::Ratio(4, 10),
                ]
                    .as_ref(),
            )
            .horizontal_margin(0)
            .vertical_margin(0)
            .split(area);

        //charts
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Max(5),
                    Constraint::Max(5),
                    Constraint::Length(1),
                ]
                    .as_ref(),
            )
            .horizontal_margin(0)
            .vertical_margin(0)
            .split(main_chunk[0]);

        for i in main_chunk[0].left() + 2..main_chunk[0].right().wrapping_sub(7) {
            buf.get_mut(i, 5)
                .set_style(self.theme.cpu_chart_line)
                .set_symbol(MIN_DOT_SYMBOL);
        }

        self.cpu_sys.render(chunks[1], buf);
        self.cpu_user.render(chunks[2], buf);
        self.throughput.render(main_chunk[1], buf);

        let spans = vec![
            Spans::from(
                vec![
                    Span::styled(format!("{}", tui::symbols::DOT), self.theme.cpu_sys_cpu_dataset),
                    Span::styled(format!("sys cpu: "), self.theme.cpu_sys_cpu_1_text),
                    Span::styled(format!("{:>5.02}%", self.last_delta_cpu_sys), self.theme.cpu_sys_cpu_2_text),
                    Span::styled(format!("{:>4}", tui::symbols::DOT), self.theme.cpu_user_cpu_dataset),
                    Span::styled(format!("user cpu: "), self.theme.cpu_user_cpu_1_text),
                    Span::styled(format!("{:>5.02}%", self.last_delta_cpu_user), self.theme.cpu_user_cpu_2_text),
                ])];
        Paragraph::new(spans).render(Rect::new(chunks[3].x + 2, chunks[3].y, chunks[3].width, chunks[3].height), buf);
    }
}

impl<'a> Updatable<&Metric> for Cpu<'a> {
    fn update(&mut self, metric: &Metric) {
        self.cpu_sys.update(&metric);
        self.cpu_user.update(&metric);
        self.throughput.update(&metric);

        self.last_delta_cpu_sys = metric.cpu.last_delta_cpu_sys;
        self.last_delta_cpu_user = metric.cpu.last_delta_cpu_user;
    }
}