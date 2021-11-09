use super::App;
use tui::{Frame, backend::Backend, layout::{Alignment, Rect}, style::{Color, Style}, text::{Spans, Text}, widgets::{Block, BorderType, Borders, Paragraph}};

pub fn draw_dashboard<B>(f: &mut Frame<B>, app: &App, chunk: Rect)
where
    B: Backend,
{
    let text = Text::from(Spans::from("DASHBOARD"));
    let widget = Paragraph::new(text).block(
        Block::default()
            .title("Dashboard")
            .title_alignment(Alignment::Right)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
    );

    f.render_widget(widget, chunk);
}