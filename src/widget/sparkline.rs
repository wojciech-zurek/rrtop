use tui::widgets::{Block, Widget};
use tui::style::Style;
use tui::symbols;
use tui::layout::Rect;
use tui::buffer::Buffer;
use std::cmp::min;
use tui::symbols::bar;

#[derive(Debug, Clone)]
pub struct Sparkline<'a> {
    /// A block to wrap the widget in
    block: Option<Block<'a>>,
    /// Widget style
    style: Style,
    /// A slice of the data to display
    data: &'a [u64],
    /// The maximum value to take to compute the maximum bar height (if nothing is specified, the
    /// widget uses the max of the dataset)
    max: Option<u64>,
    /// A set of bar symbols used to represent the give data
    bar_set: symbols::bar::Set,
    show_baseline: bool,
    fill_baseline: bool,
    direction: RenderDirection,
}

#[derive(Debug, Clone)]
pub enum RenderDirection {
    LeftToRight,
    RightToLeft,
}


impl<'a> Default for Sparkline<'a> {
    fn default() -> Sparkline<'a> {
        Sparkline {
            block: None,
            style: Default::default(),
            data: &[],
            max: None,
            bar_set: symbols::bar::NINE_LEVELS,
            show_baseline: false,
            fill_baseline: false,
            direction: RenderDirection::LeftToRight
        }
    }
}

impl<'a> Sparkline<'a> {
    pub fn block(mut self, block: Block<'a>) -> Sparkline<'a> {
        self.block = Some(block);
        self
    }

    pub fn style(mut self, style: Style) -> Sparkline<'a> {
        self.style = style;
        self
    }

    pub fn data(mut self, data: &'a [u64]) -> Sparkline<'a> {
        self.data = data;
        self
    }

    pub fn max(mut self, max: u64) -> Sparkline<'a> {
        self.max = Some(max);
        self
    }

    pub fn bar_set(mut self, bar_set: symbols::bar::Set) -> Sparkline<'a> {
        self.bar_set = bar_set;
        self
    }

    pub fn show_baseline(mut self, show_baseline: bool) -> Sparkline<'a> {
        self.show_baseline = show_baseline;
        self
    }

    pub fn fill_baseline(mut self, fill_baseline: bool) -> Sparkline<'a> {
        self.fill_baseline = fill_baseline;
        self
    }

    pub fn direction(mut self, direction: RenderDirection) -> Sparkline<'a> {
        self.direction = direction;
        self
    }
}

impl<'a> Widget for Sparkline<'a> {
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
            for i in spark_area.left()..spark_area.right() {
                buf.get_mut(i, spark_area.bottom() - 1)
                    .set_symbol(bar::ONE_EIGHTH)
                    .set_fg(self.style.fg.unwrap())
                    .set_bg(self.style.bg.unwrap());
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