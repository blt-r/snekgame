use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};

use crate::game::{Dir, GameState};

pub struct InputBuffer {
    size: u8,
    internal: [Dir; Self::CAP as usize],
}

impl InputBuffer {
    const CAP: u8 = 5;

    pub fn new() -> Self {
        Self {
            internal: [Dir::Right; Self::CAP as usize],
            size: 0,
        }
    }

    fn as_slice(&self) -> &[Dir] {
        &self.internal[..self.size as usize]
    }

    fn last(&self) -> Option<Dir> {
        self.as_slice().last().copied()
    }

    /// Pushes element if buffer is not full
    fn enqueue(&mut self, input: Dir) {
        if self.size < Self::CAP {
            self.internal[self.size as usize] = input;
            self.size += 1;
        }
    }

    fn dequeue(&mut self) -> Option<Dir> {
        if self.size == 0 {
            return None;
        }
        let first = self.internal[0];
        self.size -= 1;
        for i in 0..self.size as usize {
            self.internal[i] = self.internal[i + 1];
        }
        Some(first)
    }

    pub fn buffer_input(&mut self, game: &GameState, input: Dir) {
        let current_dir = self.last().unwrap_or(game.snake_dir());

        if input.is_perpendicular(current_dir) {
            self.enqueue(input);
        }
    }

    pub fn turn_to_do(&mut self) -> Option<Dir> {
        // If all previously queued turns are made by the snake,
        // there will be no invalid turns in the queue
        self.dequeue()
    }
}

pub enum Input {
    _Pause,
    Resize,
    Move(Dir),
    Quit,
}

fn handle_event(e: &Event) -> Option<Input> {
    match e {
        Event::Resize(_, _) => Some(Input::Resize),
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('w') | KeyCode::Up => Some(Input::Move(Dir::Up)),
            KeyCode::Char('s') | KeyCode::Down => Some(Input::Move(Dir::Down)),
            KeyCode::Char('a') | KeyCode::Left => Some(Input::Move(Dir::Left)),
            KeyCode::Char('d') | KeyCode::Right => Some(Input::Move(Dir::Right)),
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                Some(Input::Quit)
            }
            KeyCode::Char('q') => Some(Input::Quit),
            _ => None,
        },

        _ => None,
    }
}

pub fn get_input() -> eyre::Result<Option<Input>> {
    if !event::poll(Duration::ZERO)? {
        return Ok(None);
    }

    Ok(handle_event(&event::read()?))
}
