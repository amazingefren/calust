mod app;
mod event;
mod page;
mod ui;

use app::App;

use crossterm::event::{self as cevent, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::error::Error;
use std::io;
use std::time::{Duration, Instant};
use tui::backend::{Backend, CrosstermBackend};
use tui::Terminal;
use ui::draw_ui;

fn run<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| draw_ui(f, &mut app))?;
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = cevent::read()? {
                match key.code {
                    KeyCode::Esc => app.input_mode = app::InputMode::Normal,
                    KeyCode::Char('i') => app.input_mode = app::InputMode::Insert,
                    KeyCode::Char('q') => match app.input_mode {
                        app::InputMode::Normal => app.should_quit = true,
                        app::InputMode::Insert => {}
                    },
                    KeyCode::Char('h') => match app.input_mode {
                        app::InputMode::Normal => app.focus = app::Focus::Sidebar,
                        app::InputMode::Insert => {}
                    },
                    KeyCode::Char('l') => match app.input_mode {
                        app::InputMode::Normal => app.focus = app::Focus::Page,
                        app::InputMode::Insert => {}
                    },

                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
        if app.should_quit {
            return Ok(());
        }
    }
}

// Process Init
fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let tick_rate = Duration::from_millis(60);
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new("Hi", true);
    let res = run(&mut terminal, app, tick_rate);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
