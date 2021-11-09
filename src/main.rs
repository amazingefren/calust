mod app;
mod page;
mod ui;

use app::{App, InputMode};

use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::error::Error;
use std::time::{Duration, Instant};
use std::{io, thread};
use tui::backend::{Backend, CrosstermBackend};
use tui::Terminal;
use ui::draw_ui;

use std::sync::mpsc::{self, Sender};

enum Event {
    Tick,
}

fn run<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    // tick_rate: Duration,
) 
// -> io::Result<()> 
{
    // terminal.get_frame();
    // let mut last_tick = Instant::now();
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(60);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            // terminal.draw(|f| ui(f, &mut app))?;
            terminal.draw(|f| draw_ui(f, &mut app))?;
            // let timeout = tick_rate
            //     .checked_sub(last_tick.elapsed())
            //     .unwrap_or_else(|| Duration::from_secs(0));
            match rx.recv()?{
                Event::Tick=>{}
            }
            if let CEvent::Key(key) = event::read()? {
                match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('q') => {
                            return Ok(());
                        }
                        KeyCode::Char('i') => {
                            app.input_mode = InputMode::Insert;
                        }
                        _ => {}
                    },
                    InputMode::Insert => match key.code {
                        // KeyCode::Enter => {
                        // app.messages.push(app.input.drain(..).collect());
                        // }
                        KeyCode::Char(c) => {
                            app.input.push(c);
                        }
                        KeyCode::Backspace => {
                            app.input.pop();
                        }
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                        }
                        _ => {}
                    },
                }
            }
            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });
}

// Process Init
fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let tick_rate = Duration::from_millis(100);
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new("Hi", true);
    let res = run(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    // if let Err(err) = res {
    //     println!("{:?}", err)
    // }

    Ok(())
}

// fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
//     let chunks = Layout::default()
//         .direction(tui::layout::Direction::Horizontal)
//         .margin(0)
//         .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
//         .split(f.size());
//
//     // let chunks = Layout::default()
//     //     .direction(Direction::Vertical)
//     //     .margin(2)
//     //     .constraints(
//     //         [
//     //             Constraint::Length(1),
//     //             Constraint::Length(3),
//     //             Constraint::Min(1),
//     //         ]
//     //         .as_ref(),
//     //     )
//     //     .split(f.size());
//
//     let (msg, style) = match app.input_mode {
//         InputMode::Normal => (
//             vec![Span::raw("Normal")],
//             Style::default().add_modifier(Modifier::RAPID_BLINK),
//         ),
//         InputMode::Insert => (vec![Span::raw("Insert")], Style::default()),
//     };
//
//     let mut text = Text::from(Spans::from(msg));
//     text.patch_style(style);
//     let help_message = Paragraph::new(text);
//     f.render_widget(help_message, chunks[0]);
//
//     let input = Paragraph::new(app.input.as_ref())
//         .style(match app.input_mode {
//             InputMode::Normal => Style::default(),
//             InputMode::Insert => Style::default().fg(Color::Yellow),
//         })
//         .block(Block::default().borders(Borders::ALL).title("input"));
//
//     f.render_widget(input, chunks[1]);
//
//     match app.input_mode {
//         InputMode::Normal => {}
//         InputMode::Insert => f.set_cursor(chunks[1].x + 1, chunks[1].y + 1),
//     }
// }
