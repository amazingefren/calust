mod layout;
mod pages;
mod sidebar;

use self::layout::draw_layout;
use super::app::App;

use tui::{backend::Backend, Frame};

pub enum Ui {
    Dashboard,
}

pub fn draw_ui<B>(f: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    draw_layout(f, app)
}
