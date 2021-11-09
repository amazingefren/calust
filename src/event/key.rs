use crossterm::event::KeyCode;

pub enum Key {
    Enter,
    Tab,
    Backspace,
    Esc,
    Left,
    Right,
    Up,
    Down,
    Unknown,
    Char(char),
}

impl From<KeyCode> for Key {
    fn from(key_event: KeyCode) -> Self {
        match key_event {
            KeyCode::Enter => Key::Enter,
            KeyCode::Tab => Key::Tab,
            KeyCode::Backspace => Key::Backspace,
            KeyCode::Esc => Key::Esc,
            KeyCode::Left => Key::Left,
            KeyCode::Right => Key::Right,
            KeyCode::Up => Key::Up,
            KeyCode::Down => Key::Down,

            KeyCode::Char(n) => Key::Char(n),

            _ => Key::Unknown,
        }
    }
}
