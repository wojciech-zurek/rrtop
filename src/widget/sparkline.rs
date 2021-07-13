use tui::widgets::{Block, Widget};
use tui::style::Style;
use tui::symbols;
use tui::layout::Rect;
use tui::buffer::Buffer;
use std::cmp::min;
use crate::widget::LINE_SYMBOL;

#[derive(Debug, Clone)]
pub struct Sparkline<'a, 'b> {
    /// A block to wrap the widget in
    block: Option<Block<'a>>,
    /// Widget style
    style: Style,
    baseline_style: Option<Style>,
    /// A slice of the data to display
    data: &'a [u64],
    /// The maximum value to take to compute the maximum bar height (if nothing is specified, the
    /// widget uses the max of the dataset)
    max: Option<u64>,
    /// A set of bar symbols used to represent the give data
    bar_set: symbols::bar::Set,
    show_baseline: bool,
    fill_baseline: bool,
    baseline_symbol: &'b str,
    direction: RenderDirection,
}

#[derive(Debug, Clone)]
pub enum RenderDirection {
    LeftToRight,
    RightToLeft,
}


impl<'a, 'b> Default for Sparkline<'a, 'b> {
    fn default() -> Sparkline<'a, 'b> {
        Sparkline {
            block: None,
            style: Default::default(),
            baseline_style: None,
            data: &[],
            max: None,
            bar_set: symbols::bar::NINE_LEVELS,
            show_baseline: false,
            fill_baseline: false,
            baseline_symbol: LINE_SYMBOL,
            direction: RenderDirection::LeftToRight,
        }
    }
}

impl<'a, 'b> Sparkline<'a, 'b> {
    #[allow(dead_code)]
    pub fn block(mut self, block: Block<'a>) -> Sparkline<'a, 'b> {
        self.block = Some(block);
        self
    }

    #[allow(dead_code)]
    pub fn style(mut self, style: Style) -> Sparkline<'a, 'b> {
        self.style = style;
        self
    }

    #[allow(dead_code)]
    pub fn data(mut self, data: &'a [u64]) -> Sparkline<'a, 'b> {
        self.data = data;
        self
    }

    #[allow(dead_code)]
    pub fn max(mut self, max: u64) -> Sparkline<'a, 'b> {
        self.max = Some(max);
        self
    }

    #[allow(dead_code)]
    pub fn bar_set(mut self, bar_set: symbols::bar::Set) -> Sparkline<'a, 'b> {
        self.bar_set = bar_set;
        self
    }

    #[allow(dead_code)]
    pub fn show_baseline(mut self, show_baseline: bool) -> Sparkline<'a, 'b> {
        self.show_baseline = show_baseline;
        self
    }

    #[allow(dead_code)]
    pub fn baseline_style(mut self, style: Style) -> Sparkline<'a, 'b> {
        self.baseline_style = Some(style);
        self
    }

    #[allow(dead_code)]
    pub fn baseline_symbol(mut self, symbol: &'b str) -> Sparkline<'a, 'b> {
        self.baseline_symbol = symbol;
        self
    }

    #[allow(dead_code)]
    pub fn fill_baseline(mut self, fill_baseline: bool) -> Sparkline<'a, 'b> {
        self.fill_baseline = fill_baseline;
        self
    }

    #[allow(dead_code)]
    pub fn direction(mut self, direction: RenderDirection) -> Sparkline<'a, 'b> {
        self.direction = direction;
        self
    }
}

impl<'a, 'b> Widget for Sparkline<'a, 'b> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let spark_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        if spark_area.height < 1 {
            return;
        }

        if self.fill_baseline {
            let style = match self.baseline_style {
                Some(s) => s,
                None => Style::default()
            };

            for i in spark_area.left()..spark_area.right() {
                buf.get_mut(i, spark_area.bottom() - 1)
                    .set_symbol(self.baseline_symbol)
                    .set_style(style);
            }
        }

        let max = match self.max {
            Some(v) => v,
            None => *self.data.iter().max().unwrap_or(&1u64),
        };
        let max_index = min(spark_area.width as usize, self.data.len());

        let offset = if self.show_baseline { 1 } else { 0 };
        let mut data = self
            .data
            .iter()
            .take(max_index)
            .map(|e| {
                if max != 0 {
                    e * u64::from(spark_area.height) * 8 / max + offset
                } else {
                    offset
                }
            })
            .collect::<Vec<u64>>();
        for j in (0..spark_area.height).rev() {
            for (i, d) in data.iter_mut().enumerate() {
                let symbol = match *d {
                    0 => self.bar_set.empty,
                    1 => self.bar_set.one_eighth,
                    2 => self.bar_set.one_quarter,
                    3 => self.bar_set.three_eighths,
                    4 => self.bar_set.half,
                    5 => self.bar_set.five_eighths,
                    6 => self.bar_set.three_quarters,
                    7 => self.bar_set.seven_eighths,
                    _ => self.bar_set.full,
                };

                let area = match self.direction {
                    RenderDirection::LeftToRight => spark_area.left() + i as u16,
                    RenderDirection::RightToLeft => spark_area.right() - i as u16 - 1,
                };

                buf.get_mut(area, spark_area.top() + j)
                    .set_symbol(symbol)
                    .set_style(self.style);

                if *d > 8 {
                    *d -= 8;
                } else {
                    *d = 0;
                }
            }
        }
    }
}