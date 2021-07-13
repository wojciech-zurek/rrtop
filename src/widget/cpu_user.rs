use std::collections::VecDeque;
use crate::colorscheme::theme::Theme;
use crate::widget::cpu::Cpu;
use tui::widgets::{Widget, Dataset, GraphType, Chart, Axis};

use tui::layout::Rect;
use tui::buffer::Buffer;
use tui::symbols::Marker;
use crate::update::Updatable;
use crate::metric::Metric;

pub struct CpuUser<'a> {
    cpu_user: VecDeque<(f64, f64)>,
    last_delta_cpu_user: f64,
    theme: &'a Theme,
    max_elements: usize,
    update_count: u64,
}

impl<'a> CpuUser<'a> {
    pub fn new(theme: &'a Theme, max_elements: usize) -> Self {
        CpuUser {
            cpu_user: VecDeque::with_capacity(max_elements),
            last_delta_cpu_user: 0.0,
            theme,
            max_elements,
            update_count: 0,
        }
    }

    fn min_cpu_user(&self, size: usize) -> f64 {
        Cpu::min(&self.cpu_user, size)
    }

    fn max_cpu_user(&self, size: usize) -> f64 {
        Cpu::max(&self.cpu_user, size)
    }
}

impl<'a> Widget for &CpuUser<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let cpu_user = self.cpu_user.iter().map(|it| (it.0, it.1.log2() * -1.0)).collect::<Vec<(f64, f64)>>();
        let min_cpu_user = &self.min_cpu_user(area.width as usize);
        let max_cpu_user = &self.max_cpu_user(area.width as usize);

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
            .render(Rect::new(area.x + 2, area.y, area.width - 8, area.height), buf);

        let label = format!("{:.02}", min_cpu_user);
        buf.set_string(
            area.width - 1 - label.len() as u16,
            area.y,
            label,
            self.theme.cpu_border,
        );

        let label = format!("{:.02}", max_cpu_user);
        buf.set_string(
            area.width - 1 - label.len() as u16,
            area.y + 4,
            label,
            self.theme.cpu_border,
        );
    }
}

impl<'a> Updatable<&Metric> for CpuUser<'a> {
    fn update(&mut self, metric: &Metric) {
        self.update_count += 1;

        self.last_delta_cpu_user = metric.cpu.last_delta_cpu_user;

        if self.cpu_user.len() >= self.max_elements {
            self.cpu_user.pop_back();
        }

        self.cpu_user.push_front((self.update_count as f64, metric.cpu.last_delta_cpu_user));
    }
}