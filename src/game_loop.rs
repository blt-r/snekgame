use crate::{
    game::{GameState, GameStatus},
    input::{self, Input, InputBuffer},
    render::Renderer,
    themes::FullTheme,
};

use std::time::Duration;
use std::time::Instant;

struct Clock {
    frame_start: Instant, // instant at which the current frame started
}

impl Clock {
    fn new() -> Self {
        Self {
            frame_start: Instant::now(),
        }
    }

    fn frame_end(&mut self, expected_frametime: Duration) {
        let now = Instant::now();
        let since_frame_start = now.duration_since(self.frame_start);

        if since_frame_start >= expected_frametime {
            //self.dt = since_frame_start;
            self.frame_start = now;
        } else {
            std::thread::sleep(expected_frametime - since_frame_start);
            let after_sleep = Instant::now();

            //self.dt = after_sleep.duration_since(self.frame_start);
            self.frame_start = after_sleep;
        }
    }
}

pub fn run(mut game: GameState, theme: &FullTheme) -> eyre::Result<()> {
    let mut renderer = Renderer::init(game.width(), game.height())?;

    let mut input_buf = InputBuffer::new();

    let mut clock = Clock::new();

    'game_loop: loop {
        while let Some(event) = input::get_input()? {
            match event {
                Input::_Pause => todo!(),
                Input::Resize => renderer.queue_clear(),
                Input::Move(dir) => input_buf.buffer_input(&game, dir),
                Input::Quit => break 'game_loop,
            }
        }

        game.make_step(input_buf.turn_to_do());

        match game.status() {
            GameStatus::Dead | GameStatus::Win => break,
            GameStatus::Ongoing => (),
        }

        renderer.render_game(&game, theme)?;

        clock.frame_end(game.expected_frametime());
    }

    drop(renderer);

    // the renderer does not print a new line after last line,
    // because that would force terminals to have an extra
    // empty line at the bottom of the screen
    println!();

    if game.status() == GameStatus::Win {
        println!("You Won!");
    }

    Ok(())
}
