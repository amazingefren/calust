use crate::page::Page;

pub enum Focus {
    Sidebar,
    Page,
}

pub enum InputMode {
    Normal,
    Insert,
}

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub enhanced_graphics: bool,
    pub progress: f64,

    // State To Change
    pub input: String,
    pub input_mode: InputMode,
    pub page: Page,
    pub focus: Focus,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool) -> App<'a> {
        App {
            title,
            should_quit: false,
            enhanced_graphics,
            progress: 0.0,

            // State
            input: String::new(),
            input_mode: InputMode::Normal,
            page: Page::Dashboard,
            focus: Focus::Sidebar,
        }
    }
}
