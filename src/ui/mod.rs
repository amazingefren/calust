mod layout;
mod sidebar;

use self::layout::draw_layout;
use super::app::App;

use tui::{backend::Backend, Frame};

pub fn draw_ui<B>(f: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    draw_layout(f, app)
}
