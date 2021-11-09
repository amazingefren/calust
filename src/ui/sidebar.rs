use super::App;
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    text::{Spans, Text},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub fn draw_sidebar<B>(f: &mut Frame<B>, _app: &App, chunk: Rect, focus: bool)
where
    B: Backend,
{
    let text = Text::from(Spans::from("Sidebar"));
    let widget = Paragraph::new(text).block(
        Block::default()
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(if focus { Color::Red } else { Color::Reset })),
    );

    f.render_widget(widget, chunk);
}
