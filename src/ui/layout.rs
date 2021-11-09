use super::{sidebar::draw_sidebar, App};
use crate::{
    app::{Focus, InputMode},
    page::draw_page,
};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Spans, Text},
    widgets::Paragraph,
    Frame,
};

fn draw_error<B>(f: &mut Frame<B>, err: &str)
where
    B: Backend,
{
    f.render_widget(
        Paragraph::new(Text::from(Spans::from(err))),
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100)])
            .split(f.size())[0],
    )
}

pub fn draw_status<B>(f: &mut Frame<B>, app: &App, chunk: Rect)
where
    B: Backend,
{
    let fg_color;
    let text = Text::from(Spans::from(match app.input_mode {
        InputMode::Normal => {
            fg_color = Color::Reset;
            "Normal"
        }
        InputMode::Insert => {
            fg_color = Color::Green;
            "Insert"
        }
    }));

    let widget = Paragraph::new(text).style(Style::default().fg(fg_color));
    f.render_widget(widget, chunk)
}

pub fn draw_layout<B>(f: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let f_size = f.size();
    let status_height: u16 = 1;
    let f_height_safe: u16;

    // Safety
    if f_size.height > status_height {
        f_height_safe = f_size.height - status_height;
        let main_chunks = Layout::default()
            .margin(0)
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100)])
            .split(Rect {
                x: f_size.x,
                y: f_size.y,
                height: f_height_safe,
                width: f_size.width,
            });

        let top_chunks = Layout::default()
            .margin(0)
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(main_chunks[0]);

        let bot_chunks = Layout::default()
            .margin(0)
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)])
            .split(Rect {
                x: f_size.x,
                y: f_height_safe,
                height: status_height,
                width: f_size.width,
            });

        let mut page_focus: bool = false;
        let mut side_focus: bool = false;
        match app.focus {
            Focus::Page => page_focus = true,
            Focus::Sidebar => side_focus = true,
        }
        draw_sidebar(f, app, top_chunks[0], side_focus);
        draw_page(f, app, top_chunks[1], page_focus);
        draw_status(f, app, bot_chunks[0]);
    } else {
        draw_error(f, "Y SO SMALL")
    };
}
