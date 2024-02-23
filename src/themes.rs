// TODO: custom themes

pub struct FullTheme {
    pub food: FoodTheme,
    pub board: BoardTheme,
    pub snake: SnakeTheme,
    pub display_score: bool,
}

/// Constructs [`Vec<Cow<'static, str>>`]
macro_rules! cow_vec {
    [] => { Vec::new() };
    [$($x:expr),+ $(,)?] => {
        vec![$(Cow::Borrowed($x)),+]
    };
}

use std::{borrow::Cow, io::Write};

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum FoodBuiltin {
    Emoji,
    Ascii,
    Star,
    Armenian,
    Greek,
    Retro,
    Braille,
    Math,
    Chess,
}

pub struct FoodTheme {
    theme: Vec<Cow<'static, str>>,
    colors: Vec<crossterm::style::Color>,
}

impl From<FoodBuiltin> for FoodTheme {
    fn from(builtin: FoodBuiltin) -> Self {
        use crossterm::style::Color as C;
        match builtin {
            FoodBuiltin::Emoji => FoodTheme {
                theme: cow_vec![
                    "🍎", "🍇", "🍈", "🍉", "🍊", "🍋", "🍌", "🍍", "🥭", "🍏", "🍐", "🍑", "🍒",
                    "🍓", "🥝", "🍅", "🌽", "🧀", "🍪", "🍰", "🧁", "🥧",
                ],
                colors: cow_vec![],
            },
            FoodBuiltin::Ascii => FoodTheme {
                theme: cow_vec!["<>", "$$", "{}", "<3", "()", ";;", "&&", "%%", "69"],
                colors: vec![C::Blue, C::Cyan, C::Green, C::Magenta, C::Yellow, C::Red],
            },
            FoodBuiltin::Star => FoodTheme {
                theme: cow_vec!["★ "],
                colors: vec![C::Blue, C::Cyan, C::Magenta, C::Yellow, C::Red],
            },
            FoodBuiltin::Armenian => FoodTheme {
                theme: cow_vec![
                    "ա ", "բ ", "գ ", "դ ", "ե ", "զ ", "է ", "ը ", "թ ", "ժ ", "ի ", "լ ", "խ ",
                    "ծ ", "կ ", "հ ", "ձ ", "ղ ", "ճ ", "մ ", "յ ", "ն ", "շ ", "ո ", "չ ", "պ ",
                    "ջ ", "ռ ", "ս ", "վ ", "տ ", "ր ", "ց ", "ու", "փ ", "ք ", "օ ", "ֆ ", "և ",
                ],
                colors: vec![C::Blue, C::Cyan, C::Green, C::Magenta, C::Yellow, C::Red],
            },
            FoodBuiltin::Greek => FoodTheme {
                theme: cow_vec![
                    "α ", "β ", "γ ", "δ ", "ε ", "ζ ", "η ", "θ ", "ι ", "κ ", "λ ", "μ ", "ν ",
                    "ξ ", "ο ", "π ", "ρ ", "ς ", "σ ", "τ ", "υ ", "φ ", "χ ", "ψ ", "ω ",
                ],
                colors: vec![C::Blue, C::Cyan, C::Green, C::Magenta, C::Yellow, C::Red],
            },
            FoodBuiltin::Retro => FoodTheme {
                theme: cow_vec!["██"],
                colors: vec![C::Red],
            },
            FoodBuiltin::Braille => FoodTheme {
                theme: cow_vec!["⢾⡷", "⢎⡱", "⡱⢎", "⣏⣹"],
                colors: vec![C::Blue, C::Cyan, C::Green, C::Magenta, C::Yellow, C::Red],
            },
            FoodBuiltin::Math => FoodTheme {
                theme: cow_vec![
                    "∫ ", "∬ ", "∭ ", "⨌ ", "∀ ", "∃ ", "∈ ", "∑ ", "∞ ", "∅ ", "⊆ ", "≥ ", "≈ ",
                    "∆x", "∆y", "⇌ ", "± ", "≽ ", "≡ ", "ℝ ", "ℂ ", "ƒ′"
                ],
                colors: vec![C::Blue, C::Cyan, C::Green, C::Magenta, C::Yellow],
            },
            FoodBuiltin::Chess => FoodTheme {
                theme: cow_vec![
                    // "♔ ", "♕ ", "♖ ", "♗ ", "♘ ", "♙ ", // white pieces
                    "♚ ", "♛ ", "♜ ", "♝ ", "♞ ", "♟ ", // black pieces
                ],
                colors: vec![],
            },
        }
    }
}

impl FoodTheme {
    pub fn display_with_id(
        &self,
        mut f: impl Write,
        id: usize,
        color: bool,
    ) -> std::io::Result<()> {
        let food_id = id & 0x0000FFFF; // mask off lower 16 bits
        let food = self.theme[food_id % self.theme.len()].as_ref();

        if color && !self.colors.is_empty() {
            let color_id = id >> 16; // mask off upper 16 bits
            let color = self.colors[color_id % self.colors.len()];

            use crossterm::style::Stylize;
            write!(f, "{}", food.with(color))?;
        } else {
            write!(f, "{}", food)?;
        }

        Ok(())
    }
}

pub struct BorderTheme {
    pub horizontal: Cow<'static, str>,
    pub vertical: Cow<'static, str>,
    pub top_left: Cow<'static, str>,
    pub top_right: Cow<'static, str>,
    pub bottom_left: Cow<'static, str>,
    pub bottom_right: Cow<'static, str>,
}

pub struct BoardTheme {
    pub border: Option<BorderTheme>,
    pub empty: Cow<'static, str>,
}

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum BoardBuiltin {
    Double,
    Rounded,
    Ascii,
    Classic,
    Empty,
    Retro,
}

impl From<BoardBuiltin> for BoardTheme {
    fn from(builtin: BoardBuiltin) -> Self {
        match builtin {
            BoardBuiltin::Double => BoardTheme {
                border: Some(BorderTheme {
                    horizontal: "═".into(),
                    vertical: "║".into(),
                    top_left: "╔".into(),
                    top_right: "╗".into(),
                    bottom_left: "╚".into(),
                    bottom_right: "╝".into(),
                }),
                empty: "  ".into(),
            },
            BoardBuiltin::Rounded => BoardTheme {
                border: Some(BorderTheme {
                    horizontal: "─".into(),
                    vertical: "│".into(),
                    top_left: "╭".into(),
                    top_right: "╮".into(),
                    bottom_left: "╰".into(),
                    bottom_right: "╯".into(),
                }),
                empty: "  ".into(),
            },
            BoardBuiltin::Ascii => BoardTheme {
                border: Some(BorderTheme {
                    horizontal: "-".into(),
                    vertical: "|".into(),
                    top_left: "*".into(),
                    top_right: "*".into(),
                    bottom_left: "*".into(),
                    bottom_right: "*".into(),
                }),
                empty: "  ".into(),
            },
            BoardBuiltin::Classic => BoardTheme {
                border: None,
                empty: "` ".into(),
            },
            BoardBuiltin::Empty => BoardTheme {
                border: None,
                empty: "  ".into(),
            },
            BoardBuiltin::Retro => BoardTheme {
                border: None,
                empty: "░░".into(),
            },
        }
    }
}

#[derive(Debug)]
pub struct SnakeTheme {
    pub head_up: Cow<'static, str>,
    pub head_down: Cow<'static, str>,
    pub head_left: Cow<'static, str>,
    pub head_right: Cow<'static, str>,

    pub tail_up: Cow<'static, str>,
    pub tail_down: Cow<'static, str>,
    pub tail_left: Cow<'static, str>,
    pub tail_right: Cow<'static, str>,

    pub body_vertical: Cow<'static, str>,
    pub body_horizontal: Cow<'static, str>,
    pub body_up_right: Cow<'static, str>,
    pub body_down_right: Cow<'static, str>,
    pub body_up_left: Cow<'static, str>,
    pub body_down_left: Cow<'static, str>,
}

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum SnakeBuiltin {
    // TODO: more built-in snake themes
    Braille,
    Line,
    Retro,
    Basic,
}

impl From<SnakeBuiltin> for SnakeTheme {
    fn from(builtin: SnakeBuiltin) -> Self {
        match builtin {
            SnakeBuiltin::Braille => SnakeTheme {
                head_up: "⢰⡆".into(),
                head_down: "⠸⠇".into(),
                head_left: "⠰⠶".into(),
                head_right: "⠶⠆".into(),
                tail_up: "⢰⡀".into(),
                tail_down: "⠈⠇".into(),
                tail_left: "⠠⠴".into(),
                tail_right: "⠖⠂".into(),
                body_vertical: "⢸⡇".into(),
                body_horizontal: "⠶⠶".into(),
                body_up_right: "⢶⡆".into(),
                body_down_right: "⠾⠇".into(),
                body_up_left: "⢰⡶".into(),
                body_down_left: "⠸⠷".into(),
            },
            SnakeBuiltin::Line => SnakeTheme {
                head_up: "╻ ".into(),
                head_down: "╹ ".into(),
                head_left: " ━".into(),
                head_right: "━ ".into(),
                tail_up: "╻ ".into(),
                tail_down: "╹ ".into(),
                tail_left: " ━".into(),
                tail_right: "━ ".into(),
                body_vertical: "┃ ".into(),
                body_horizontal: "━━".into(),
                body_up_right: "┓ ".into(),
                body_down_right: "┛ ".into(),
                body_up_left: "┏━".into(),
                body_down_left: "┗━".into(),
            },
            SnakeBuiltin::Basic => SnakeTheme {
                head_up: "[]".into(),
                head_down: "[]".into(),
                head_left: "[]".into(),
                head_right: "[]".into(),
                tail_up: "[]".into(),
                tail_down: "[]".into(),
                tail_left: "[]".into(),
                tail_right: "[]".into(),
                body_vertical: "[]".into(),
                body_horizontal: "[]".into(),
                body_up_right: "[]".into(),
                body_down_right: "[]".into(),
                body_up_left: "[]".into(),
                body_down_left: "[]".into(),
            },
            SnakeBuiltin::Retro => SnakeTheme {
                head_up: "██".into(),
                head_down: "██".into(),
                head_left: "██".into(),
                head_right: "██".into(),
                tail_up: "██".into(),
                tail_down: "██".into(),
                tail_left: "██".into(),
                tail_right: "██".into(),
                body_vertical: "██".into(),
                body_horizontal: "██".into(),
                body_up_right: "██".into(),
                body_down_right: "██".into(),
                body_up_left: "██".into(),
                body_down_left: "██".into(),
            },
        }
    }
}
