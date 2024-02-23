use std::io::{StdoutLock, Write};

use crossterm::{cursor, terminal, ExecutableCommand, QueueableCommand};
use ndarray::Array2;

use crate::{
    game::{Dir, GameState},
    themes::FullTheme,
};

#[derive(Clone, Copy)]
enum FieldCell {
    Empty,
    Food(usize), // stores the id of the food

    HeadUp,
    HeadDown,
    HeadLeft,
    HeadRight,

    TailUp,
    TailDown,
    TailLeft,
    TailRight,

    BodyVertical,
    BodyHorizontal,

    BodyUpRight,
    BodyDownRight,
    BodyUpLeft,
    BodyDownLeft,
}

impl FieldCell {
    // Head that looks to `dir`
    fn head_from(dir: Dir) -> Self {
        match dir {
            Dir::Up => FieldCell::HeadUp,
            Dir::Down => FieldCell::HeadDown,
            Dir::Left => FieldCell::HeadLeft,
            Dir::Right => FieldCell::HeadRight,
        }
    }

    // Tail that looks to `dir`
    fn tail_from(dir: Dir) -> Self {
        match dir {
            Dir::Up => FieldCell::TailUp,
            Dir::Down => FieldCell::TailDown,
            Dir::Left => FieldCell::TailLeft,
            Dir::Right => FieldCell::TailRight,
        }
    }

    // Body which has another body part to `dir1` and `dir2` from itself
    fn body_from(dir1: Dir, dir2: Dir) -> Self {
        match (dir1, dir2) {
            (Dir::Down, Dir::Up) | (Dir::Up, Dir::Down) => FieldCell::BodyVertical,
            (Dir::Right, Dir::Left) | (Dir::Left, Dir::Right) => FieldCell::BodyHorizontal,
            (Dir::Up, Dir::Left) | (Dir::Left, Dir::Up) => FieldCell::BodyUpLeft,
            (Dir::Down, Dir::Left) | (Dir::Left, Dir::Down) => FieldCell::BodyDownLeft,
            (Dir::Up, Dir::Right) | (Dir::Right, Dir::Up) => FieldCell::BodyUpRight,
            (Dir::Down, Dir::Right) | (Dir::Right, Dir::Down) => FieldCell::BodyDownRight,
            _ => FieldCell::BodyHorizontal,
        }
    }

    fn draw_with_theme(
        &self,
        f: &mut impl Write,
        t: &FullTheme,
        color: bool,
    ) -> Result<(), std::io::Error> {
        let str: &str = match self {
            FieldCell::Food(id) => {
                t.food.display_with_id(f, *id, color)?;
                return Ok(());
            }
            FieldCell::Empty => &t.board.empty,
            FieldCell::HeadUp => &t.snake.head_up,
            FieldCell::HeadDown => &t.snake.head_down,
            FieldCell::HeadLeft => &t.snake.head_left,
            FieldCell::HeadRight => &t.snake.head_right,
            FieldCell::BodyVertical => &t.snake.body_vertical,
            FieldCell::BodyHorizontal => &t.snake.body_horizontal,
            FieldCell::BodyUpRight => &t.snake.body_up_right,
            FieldCell::BodyDownRight => &t.snake.body_down_right,
            FieldCell::BodyUpLeft => &t.snake.body_up_left,
            FieldCell::BodyDownLeft => &t.snake.body_down_left,
            FieldCell::TailUp => &t.snake.tail_up,
            FieldCell::TailDown => &t.snake.tail_down,
            FieldCell::TailLeft => &t.snake.tail_left,
            FieldCell::TailRight => &t.snake.tail_right,
        };
        write!(f, "{}", str)
    }
}

/// This struct holds StdoutLock which means, that while it exists,
/// no other thread can write into stdout.
/// Which is fine since the program is single threaded
pub struct Renderer {
    screen: Array2<FieldCell>,
    stdout: StdoutLock<'static>,
    /// By default [std::io::Stdout] flushes on `\n` which we don't want
    /// see: https://github.com/rust-lang/libs-team/issues/148
    /// and: https://github.com/rust-lang/rust/pull/78515
    out_buf: Vec<u8>,
    color: bool,
}

impl Renderer {
    pub fn init(game_width: usize, game_height: usize) -> Result<Self, std::io::Error> {
        let mut stdout = std::io::stdout().lock();

        terminal::enable_raw_mode()?;
        stdout.queue(cursor::Hide)?;
        stdout.queue(terminal::Clear(terminal::ClearType::All))?;
        // stdout.queue(cursor::MoveTo(0, 0))?;
        stdout.flush()?;

        Ok(Self {
            screen: Array2::from_elem([game_height, game_width], FieldCell::Empty),
            stdout,
            out_buf: Vec::with_capacity(8 * 1024),
            color: !crossterm::style::Colored::ansi_color_disabled(),
        })
    }

    /// Queues clear escape sequence, so the next frame will clear the terminal
    pub fn queue_clear(&mut self) {
        self.out_buf
            .queue(terminal::Clear(terminal::ClearType::All))
            .expect("writing into Vec cannot fail");
    }

    pub fn render_game(
        &mut self,
        game: &GameState,
        theme: &FullTheme,
    ) -> Result<(), std::io::Error> {
        // === Setup the screen

        let w = game.conf.w;
        let h = game.conf.h;

        self.screen.fill(FieldCell::Empty);

        for [before, el, after] in game.snake.array_windows() {
            let dir1 = el.compare(before, w, h);
            let dir2 = el.compare(after, w, h);
            self.screen[[el.y, el.x]] = FieldCell::body_from(dir1, dir2);
        }

        self.screen[[game.snake[0].y, game.snake[0].x]] = FieldCell::head_from(game.dir);

        if let [.., before_tail, tail] = game.snake.as_slice() {
            self.screen[[tail.y, tail.x]] = FieldCell::tail_from(tail.compare(before_tail, w, h));
        }

        for food in game.food.iter() {
            self.screen[[food.pos.y, food.pos.x]] = FieldCell::Food(food.id);
        }

        // === Start writng

        let out_buf = &mut self.out_buf;
        out_buf.queue(cursor::MoveTo(0, 0))?;

        if let Some(border) = &theme.board.border {
            let cell_size = theme.board.empty.len();

            write!(
                out_buf,
                "{}{}{}",
                border.top_left,
                border.horizontal.repeat(w * cell_size),
                border.top_right
            )?;
            if theme.display_score {
                out_buf.queue(cursor::MoveToColumn(cell_size as u16))?;
                write!(out_buf, "Score: {}", game.score)?;
            }
            write!(out_buf, "\r\n")?;
            for row in self.screen.outer_iter() {
                write!(out_buf, "{}", border.vertical)?;
                for cell in row.iter() {
                    cell.draw_with_theme(out_buf, theme, self.color)?;
                }
                write!(out_buf, "{}\r\n", border.vertical)?;
            }
            write!(
                out_buf,
                "{}{}{}\r\n",
                border.bottom_left,
                border.horizontal.repeat(w * cell_size),
                border.bottom_right
            )?;
        } else {
            if theme.display_score {
                out_buf.queue(terminal::Clear(terminal::ClearType::CurrentLine))?;
                write!(out_buf, "Score: {}\r\n", game.score)?;
            }
            for row in self.screen.outer_iter() {
                for cell in row.iter() {
                    cell.draw_with_theme(out_buf, theme, self.color)?;
                }
                write!(out_buf, "\r\n")?;
            }
        }

        self.stdout.write_all(out_buf)?;
        self.stdout.flush()?;
        out_buf.clear();

        Ok(())
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        // ignore io errors
        let _ = self.stdout.execute(cursor::Show);
        let _ = terminal::disable_raw_mode();
    }
}
