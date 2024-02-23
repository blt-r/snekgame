use std::time::Duration;

use crossterm::event::{self, KeyCode, KeyModifiers};

use crate::game::{Dir, GameState};

pub struct InputBuffer {
    internal: [Dir; Self::CAP as usize],
    size: u8,
}

impl std::ops::Deref for InputBuffer {
    type Target = [Dir];

    fn deref(&self) -> &Self::Target {
        &self.internal[..self.size as usize]
    }
}

impl InputBuffer {
    const CAP: u8 = 5;

    pub fn new() -> Self {
        Self {
            internal: [Dir::Right; Self::CAP as usize],
            size: 0,
        }
    }

    /// Pushes element if buffer is not full
    pub fn enqueue(&mut self, input: Dir) {
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
        self.internal.rotate_left(1);
        self.size -= 1;
        Some(first)
    }
}

pub enum InputAction {
    None,
    Quit,
    Clear,
}

pub fn handle_inputs(buf: &mut InputBuffer, game: &GameState) -> eyre::Result<InputAction> {
    let mut clear = false;

    while event::poll(Duration::ZERO)? {
        let e = match event::read()? {
            event::Event::Key(e) => e,
            event::Event::Resize(_, _) => {
                clear = true;
                continue;
            }
            _ => continue,
        };
        if e.code == KeyCode::Char('c') && e.modifiers.contains(KeyModifiers::CONTROL) {
            return Ok(InputAction::Quit);
        }

        // TODO: add a way to customize keybinds
        match e.code {
            KeyCode::Char('w') | KeyCode::Up => buffer_input(buf, game, Dir::Up),
            KeyCode::Char('s') | KeyCode::Down => buffer_input(buf, game, Dir::Down),
            KeyCode::Char('a') | KeyCode::Left => buffer_input(buf, game, Dir::Left),
            KeyCode::Char('d') | KeyCode::Right => buffer_input(buf, game, Dir::Right),
            KeyCode::Char('q') => return Ok(InputAction::Quit),
            _ => (),
        };
    }
    if clear {
        Ok(InputAction::Clear)
    } else {
        Ok(InputAction::None)
    }
}

fn buffer_input(buf: &mut InputBuffer, game: &GameState, input: Dir) {
    if buf.size == 0 && !input.is_perpendicular(game.dir) {
        // if there's no inputs buffered and the snake cannot turn that way,
        // we don't want to buffer the input
        return;
    }

    if buf.last() == Some(&input) {
        // if there is already the same input buffered,
        // we don't want to buffer the input
        return;
    }

    buf.enqueue(input);
}

pub fn turn_to_do(buf: &mut InputBuffer, game: &GameState) -> Option<Dir> {
    while let Some(input) = buf.dequeue() {
        if input.is_perpendicular(game.dir) {
            return Some(input);
        }
    }
    None
}
