use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::Style;
use tui::symbols::line;
use tui::text::Span;
use tui::widgets::{LineGauge, Widget};

pub fn render_line_gauge<'a, >(label: Span, v2: f64, width: u16, style: Style, gauge_style: Style) -> Vec<Span<'a>> {
    let width = (width / 4).wrapping_add(5);
    let mut buffer = Buffer::empty(Rect::new(0, 0, width, 1));

    LineGauge::default()
        .style(style)
        .gauge_style(gauge_style)
        .label(label)
        .line_set(line::THICK)
        .ratio(v2)
        .render(Rect::new(0, 0, width, 1), &mut buffer);
    buffer.content.
        iter()
        .map(|it| Span::styled(it.symbol.to_owned(), Style::default().bg(it.bg).fg(it.fg).add_modifier(it.modifier)))
        .collect()
}
