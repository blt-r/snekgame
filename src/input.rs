use std::time::Duration;

use crossterm::event::{self, KeyCode, KeyModifiers};

use crate::game::{Dir, GameState};

pub struct InputBuffer {
    internal: [Dir; Self::CAP as usize],
    size: u8,
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
        if self.size == 0 && !input.is_perpendicular(game.snake_dir()) {
            // if there's no inputs buffered and the snake cannot turn that way,
            // we don't want to buffer the input
            return;
        }

        if self.as_slice().last() == Some(&input) {
            // if there is already the same input buffered,
            // we don't want to buffer the input
            return;
        }

        self.enqueue(input);
    }

    /// If theres a valid turn queued, returns that turn
    pub fn turn_to_do(&mut self, game: &GameState) -> Option<Dir> {
        while let Some(input) = self.dequeue() {
            if input.is_perpendicular(game.snake_dir()) {
                return Some(input);
            }
        }
        None
    }
}

pub enum Input {
    Pause,
    Resize,
    Move(Dir),
    Quit,
}

fn handle_event(e: event::Event) -> Option<Input> {
    match e {
        event::Event::Resize(_, _) => Some(Input::Resize),
        event::Event::Key(key) => match key.code {
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

    Ok(handle_event(event::read()?))
}
