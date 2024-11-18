use std::{
    collections::{HashSet, VecDeque},
    time::Duration,
};

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
    pub fn move_wrapping(mut self, dir: Dir, w: usize, h: usize) -> Coords {
        match dir {
            Dir::Up => self.y = (self.y + h - 1) % h,
            Dir::Down => self.y = (self.y + 1) % h,
            Dir::Left => self.x = (self.x + w - 1) % w,
            Dir::Right => self.x = (self.x + 1) % w,
        }
        self
    }

    /// moves the point in `dir` direction returning None
    /// if result is outside the `w` by `h` field.
    pub fn move_bumping(mut self, dir: Dir, w: usize, h: usize) -> Option<Coords> {
        match dir {
            Dir::Up if self.y > 0 => self.y -= 1,
            Dir::Down if self.y < h - 1 => self.y += 1,
            Dir::Left if self.x > 0 => self.x -= 1,
            Dir::Right if self.x < w - 1 => self.x += 1,
            _ => return None,
        }
        Some(self)
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
    pub height: usize,
    pub width: usize,
    pub initial_length: usize,
    pub seed: u64,
    pub solid_walls: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameStatus {
    Win,
    Dead,
    Ongoing,
}

// TODO: we want to maintain some sort of 2d array with the board that represents
// the current state of the game. It will store the postions of food,
// for constant time lookup. And it will store all the snake pieces for the
// renderer, so it doesn't need to reconstruct the board every frame.
pub struct GameState {
    conf: GameConf,
    snake: VecDeque<Coords>,
    snake_dir: Dir,
    food: Vec<Food>,
    status: GameStatus,
    score: u32,
    speed: u32,
    rng: StdRng,
}

impl GameState {
    pub fn new(conf: GameConf) -> Self {
        let y = conf.height / 2;
        let head_x = (conf.width - 1) / 2 + conf.initial_length / 2;
        let tail_x = head_x + 1 - conf.initial_length;
        let snake = (tail_x..=head_x).rev().map(|x| Coords { x, y }).collect();

        let food = Vec::new();

        let mut game = GameState {
            snake,
            snake_dir: Dir::Right,
            food,
            status: GameStatus::Ongoing,
            score: 0,
            speed: conf.initial_speed,
            rng: StdRng::seed_from_u64(conf.seed),
            conf,
        };

        for _ in 0..game.conf.food_n {
            if let Some(new) = game.find_new_food_place() {
                game.food.push(new);
            } else {
                game.status = GameStatus::Win;
                break;
            };
        }

        game
    }

    pub fn make_step(&mut self, turn: Option<Dir>) {
        if let Some(dir) = turn {
            self.snake_dir = dir;
        }

        let new_head_pos = if self.conf.solid_walls {
            match self.snake[0].move_bumping(self.snake_dir, self.conf.width, self.conf.height) {
                Some(new_head_pos) => new_head_pos,
                None => {
                    self.status = GameStatus::Dead;
                    return;
                }
            }
        } else {
            self.snake[0].move_wrapping(self.snake_dir, self.conf.width, self.conf.height)
        };

        // remember where was the tail, to place here new element if needed
        let old_tail = self.snake.pop_back().unwrap();

        if self.snake.contains(&new_head_pos) {
            self.status = GameStatus::Dead;
            return;
        }

        self.snake.push_front(new_head_pos);

        if let Some(i) = self.food.iter().position(|f| f.pos == new_head_pos) {
            self.food.remove(i);
            self.snake.push_back(old_tail);

            if let Some(new) = self.find_new_food_place() {
                self.food.push(new);
            } else {
                self.status = GameStatus::Win;
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
    }

    /// Finds new place for food, and
    /// if there is no space on the field returns `None`
    // TODO: make this function not have access to whole &mut self
    fn find_new_food_place(&mut self) -> Option<Food> {
        let mut taken_spots = HashSet::new();
        taken_spots.extend(self.snake.iter().copied());
        taken_spots.extend(self.food.iter().map(|f| f.pos));

        let spots = self.conf.height * self.conf.width - taken_spots.len();

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
            if coords.x >= self.conf.width {
                coords.x = 0;
                coords.y += 1;
            }
        }

        Some(Food {
            pos: coords,
            id: self.rng.gen(),
        })
    }

    pub fn expected_frametime(&self) -> Duration {
        Duration::from_secs(1) / self.speed
    }
    pub fn width(&self) -> usize {
        self.conf.width
    }
    pub fn height(&self) -> usize {
        self.conf.height
    }
    pub fn score(&self) -> u32 {
        self.score
    }
    pub fn snake_dir(&self) -> Dir {
        self.snake_dir
    }
    pub fn status(&self) -> GameStatus {
        self.status
    }
    pub fn snake_iter(&self) -> impl Iterator<Item = Coords> + '_ {
        self.snake.iter().copied()
    }
    pub fn food(&self) -> &[Food] {
        self.food.as_slice()
    }
}
