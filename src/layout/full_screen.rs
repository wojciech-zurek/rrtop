use tui::Frame;
use tui::layout::Rect;
use tui::widgets::Widget;

use crate::terminal::Backend;

pub fn draw<W: Widget>(f: &mut Frame<Backend>, area: Rect, widget: W) {
    f.render_widget(widget, area);
}