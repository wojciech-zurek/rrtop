use tui::Frame;
use tui::layout::Rect;

use crate::app::App;
use crate::terminal::Backend;

pub fn draw(f: &mut Frame<Backend>, area: Rect, app: &mut App) {
        f.render_stateful_widget(&app.stat, area, &mut app.stat_table_state);
}
