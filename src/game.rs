use std::{collections::HashSet, time::Duration};

use rand::{rngs::StdRng, Rng, SeedableRng};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Coords {
    pub x: usize,
    pub y: usize,
}

impl std::fmt::Debug for Coords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Good when logging a lot of these
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Coords {
    /// moves the point in `dir` direction wrapping on `w` by `h` field.
    pub fn move_towards(mut self, dir: Dir, w: usize, h: usize) -> Coords {
        match dir {
            Dir::Up => self.y = (self.y + h - 1) % h,
            Dir::Down => self.y = (self.y + 1) % h,
            Dir::Left => self.x = (self.x + w - 1) % w,
            Dir::Right => self.x = (self.x + 1) % w,
        }
        self
    }

    /// Compares two adjacent coords on `w` by `h` field, wrapping around the edges
    pub fn compare(&self, other: &Coords, w: usize, h: usize) -> Dir {
        match () {
            _ if self.x == 0 && other.x == w - 1 => Dir::Right,
            _ if self.x == w - 1 && other.x == 0 => Dir::Left,
            _ if self.x > other.x => Dir::Right,
            _ if self.x < other.x => Dir::Left,

            _ if self.y == 0 && other.y == h - 1 => Dir::Down,
            _ if self.y == h - 1 && other.y == 0 => Dir::Up,
            _ if self.y > other.y => Dir::Down,
            _ if self.y < other.y => Dir::Up,
            _ => panic!("Looks like they have the same position: {self:?}, {other:?}"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dir {
    // least significant bit represents direction (pos/neg),
    // and the next bit represents axis (x/y)
    Left = 0b00,  // 0 = x, 0 == +
    Right = 0b01, // 0 = x, 1 == -
    Up = 0b10,    // 1 = y, 0 == +
    Down = 0b11,  // 1 = y, 1 == -
}

impl Dir {
    pub fn is_perpendicular(self, other: Dir) -> bool {
        // mask off axis bits and see if they are different
        (self as u8 & 0b10) != (other as u8 & 0b10)
    }
}

#[derive(Debug)]
pub struct Food {
    pub pos: Coords,
    pub id: usize, // id is responsible for the type of food to render
}

pub struct GameConf {
    pub food_to_speed_up: u32,
    pub food_n: u32,
    pub initial_speed: u32,
    pub h: usize,
    pub w: usize,
    pub initial_length: usize,
    pub seed: u64,
}

pub struct Game {
    pub conf: GameConf,
    pub snake: Vec<Coords>,
    pub dir: Dir,
    pub food: Vec<Food>,
    pub is_dead: bool,
    pub is_win: bool,
    pub time_since_last_step: Duration,
    pub score: u32,
    pub speed: u32,
    pub rng: StdRng,
}

impl Game {
    pub fn new(conf: GameConf) -> Self {
        let y = conf.h / 2;
        let head_x = (conf.w - 1) / 2 + conf.initial_length / 2;
        let tail_x = head_x + 1 - conf.initial_length;
        let snake = (tail_x..=head_x).rev().map(|x| Coords { x, y }).collect();

        let food = Vec::new();

        let mut game = Game {
            snake,
            dir: Dir::Right,
            food,
            is_dead: false,
            is_win: false,
            time_since_last_step: Duration::ZERO,
            score: 0,
            speed: conf.initial_speed,
            rng: StdRng::seed_from_u64(conf.seed),
            conf,
        };

        for _ in 0..game.conf.food_n {
            if let Some(new) = game.new_food() {
                game.food.push(new);
            } else {
                game.is_win = true;
                break;
            };
        }

        game
    }

    pub fn expected_frametime(&self) -> Duration {
        Duration::from_secs(1) / self.speed
    }

    pub fn make_step(&mut self, turn: Option<Dir>) {
        self.time_since_last_step = Duration::ZERO;

        if let Some(dir) = turn {
            self.dir = dir;
        }

        // remember where was the last element, in case we will need to add new one
        let last = *self.snake.last().unwrap();

        // advance every element of the snake, except for the head
        for i in (1..self.snake.len()).rev() {
            self.snake[i] = self.snake[i - 1];
        }

        self.snake[0] = self.snake[0].move_towards(self.dir, self.conf.w, self.conf.h);

        if let Some(i) = self.food.iter().position(|f| f.pos == self.snake[0]) {
            self.food.remove(i);
            self.snake.push(last);

            if let Some(new) = self.new_food() {
                self.food.push(new);
            } else {
                self.is_win = true;
            }

            self.score += 1;
            if self.conf.food_to_speed_up != 0 {
                // check for overflow, because initial_speed may be set by user to u32::MAX
                self.speed = self
                    .conf
                    .initial_speed
                    .saturating_add(self.score / self.conf.food_to_speed_up);
            }
        }

        if self.snake[1..].contains(&self.snake[0]) {
            self.is_dead = true;
        }
    }

    /// Finds new place for food, and
    /// if there is no space on the field returns `None`
    fn new_food(&mut self) -> Option<Food> {
        let mut taken_spots = HashSet::new();
        taken_spots.extend(self.snake.iter().cloned());
        taken_spots.extend(self.food.iter().map(|f| f.pos));

        let spots = self.conf.h * self.conf.w - taken_spots.len();

        if spots == 0 {
            return None;
        }

        let choosen_spot = self.rng.gen_range(0..spots);
        let mut coords = Coords { x: 0, y: 0 };
        let mut i = 0;
        loop {
            if !taken_spots.contains(&coords) {
                i += 1;
            }

            if i > choosen_spot {
                break;
            }

            coords.x += 1;
            if coords.x >= self.conf.w {
                coords.x = 0;
                coords.y += 1;
            }
        }

        Some(Food {
            pos: coords,
            id: self.rng.gen(),
        })
    }
}
