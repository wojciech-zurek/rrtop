use tui::Frame;
use tui::layout::Rect;

use crate::app::App;
use crate::terminal::Backend;

pub fn draw(f: &mut Frame<Backend>, area: Rect, app: &mut App) {
        f.render_widget(&mut app.stat, area);
}
