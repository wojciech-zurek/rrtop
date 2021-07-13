use std::collections::VecDeque;
use crate::colorscheme::theme::Theme;
use crate::update::Updatable;
use tui::widgets::{Widget, Dataset, GraphType, Chart, Axis};
use tui::layout::Rect;
use tui::buffer::Buffer;
use tui::symbols::Marker;
use crate::widget::cpu::Cpu;
use crate::metric::Metric;

pub struct CpuSys<'a> {
    cpu_sys: VecDeque<(f64, f64)>,
    last_delta_cpu_sys: f64,
    theme: &'a Theme,
    max_elements: usize,
    update_count: u64,
}

impl<'a> CpuSys<'a> {
    pub fn new(theme: &'a Theme, max_elements: usize) -> Self {
        CpuSys {
            cpu_sys: VecDeque::with_capacity(max_elements),
            last_delta_cpu_sys: 0.0,
            theme,
            max_elements,
            update_count: 0,
        }
    }

    fn min_cpu_sys(&self, size: usize) -> f64 {
        Cpu::min(&self.cpu_sys, size)
    }

    fn max_cpu_sys(&self, size: usize) -> f64 {
        Cpu::max(&self.cpu_sys, size)
    }
}

impl<'a> Widget for &CpuSys<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        //cpu sys
        let cpu_sys = self.cpu_sys.iter().map(|it| (it.0, it.1.log2())).collect::<Vec<(f64, f64)>>();
        let min_cpu_sys = &self.min_cpu_sys(area.width as usize);
        let max_cpu_sys = &self.max_cpu_sys(area.width as usize);

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
            .render(Rect::new(area.x + 2, area.y, area.width - 8, area.height), buf);

        let label = format!("{:.02}", max_cpu_sys);
        buf.set_string(
            area.width - 1 - label.len() as u16,
            area.y,
            label,
            self.theme.cpu_border,
        );

        let label = format!("{:.02}", min_cpu_sys);
        buf.set_string(
            area.width - 1 - label.len() as u16,
            area.y + 4,
            label,
            self.theme.cpu_border,
        );
    }
}

impl<'a> Updatable<&Metric> for CpuSys<'a> {
    fn update(&mut self, metric: &Metric) {
        self.update_count += 1;

        self.last_delta_cpu_sys = metric.cpu.last_delta_cpu_sys;

        if self.cpu_sys.len() >= self.max_elements {
            self.cpu_sys.pop_back();
        }

        self.cpu_sys.push_front((self.update_count as f64, metric.cpu.last_delta_cpu_sys));
    }
}