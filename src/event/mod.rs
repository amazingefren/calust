mod key;
//
pub use self::key::Key;
use std::{
    sync::mpsc::{channel, Receiver, RecvError, Sender},
    thread,
    time::Duration,
};

use crossterm::event;

pub enum InputEvent {
    Input(Key),
    Tick,
}

pub struct Events {
    rx: Receiver<InputEvent>,
    tx: Sender<InputEvent>,
}

impl Events {
    pub fn new(tick_rate: Duration) -> Events {
        let (tx, rx) = channel();
        let event_tx = tx.clone();
        thread::spawn(move || {
            if crossterm::event::poll(tick_rate).unwrap() {
                if let event::Event::Key(key) = event::read().unwrap() {
                    let key = Key::from(key.code);
                    event_tx.send(InputEvent::Input(key)).unwrap();
                }
            }
        });

        Events { rx, tx }
    }

    pub fn next(&self) -> Result<InputEvent, RecvError> {
        self.rx.recv()
    }
}
