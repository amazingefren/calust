mod dashboard;

use super::app::App;
use self::dashboard::draw_dashboard;

use tui::{backend::Backend, layout::Rect, Frame};

pub enum Page {
    Dashboard,
}

pub fn draw_page<B>(f: &mut Frame<B>, app: &App, chunk: Rect)
where
    B: Backend,
{
    match app.page {
        Page::Dashboard => draw_dashboard(f, app, chunk),
    }
}
