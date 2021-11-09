use super::App;
use tui::{Frame, backend::Backend, layout::Rect, text::{Spans, Text}, widgets::{Block, BorderType, Borders, Paragraph}};

pub fn draw_sidebar<B>(f: &mut Frame<B>, app: &App, chunk: Rect)
where
    B: Backend,
{
    let text = Text::from(Spans::from("HELLO WORLD"));
    let widget = Paragraph::new(text).block(Block::default().border_type(BorderType::Rounded).borders(Borders::ALL));

    f.render_widget(widget, chunk);
}
