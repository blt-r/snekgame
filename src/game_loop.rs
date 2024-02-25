use crate::{
    game::GameState,
    input,
    input::{InputAction, InputBuffer},
    render::Renderer,
    themes::FullTheme,
};

use std::time::Duration;
use std::time::Instant;

#[derive(PartialEq)]
pub enum GameResult {
    Win,
    Lose,
    Interrupted,
}

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
    let mut renderer = Renderer::init(game.conf.w, game.conf.h)?;

    let mut input_buf = InputBuffer::new();

    let mut clock = Clock::new();

    let game_result = loop {
        match input::handle_inputs(&mut input_buf, &game)? {
            InputAction::None => (),
            InputAction::Quit => break GameResult::Interrupted,
            InputAction::Clear => {
                renderer.queue_clear();
            }
        }

        let turn = input::turn_to_do(&mut input_buf, &game);
        game.make_step(turn);

        if game.is_dead {
            break GameResult::Lose;
        } else if game.is_win {
            break GameResult::Win;
        }

        renderer.render_game(&game, theme)?;

        clock.frame_end(game.expected_frametime());
    };

    drop(renderer);

    // the renderer does not print a new line after last line,
    // because that would force terminals to have an extra
    // empty line at the bottom of the screen
    println!();

    if game_result == GameResult::Win {
        println!("You Won!");
    }

    Ok(())
}
