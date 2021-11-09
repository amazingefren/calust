use crate::page::Page;

pub struct Signal<S: Iterator> {
    source: S,
    pub points: Vec<S::Item>,
    tick_rate: usize,
}

// impl<S> Signal<S>
// where
//     S: Iterator,
// {
//     fn on_tick(&mut self) {
//         for _ in 0..self.tick_rate {
//             self.points.remove(0);
//         }
//         self.points
//             .extend(self.source.by_ref().take(self.tick_rate))
//     }
// }
//
// pub struct Signals {
//     pub sin1: Signal<S>,
//     pub sin2: Signal<S>,
//     pub window: [f64; 2],
// }
//
// impl Signals {
//     fn on_tick(&mut self) {
//         self.window[0] += 1.0;
//         self.window[1] += 1.0;
//     }
// }

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
        }
    }
    //
    // pub fn on_tick(&mut self) {
    //     self.progress += 0.001;
    //     if self.progress > 1.0 {
    //         self.progress = 0.0;
    //     }
    //     // self.
    // }
}
