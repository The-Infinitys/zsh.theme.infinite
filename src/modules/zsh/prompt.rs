mod left;
mod right;
pub use left::left;
pub use right::right;

use crate::{args::PromptSide, zsh::theme::PromptTheme};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct PromptBox {
    pub side: PromptSide,
    pub priority: u32,
    pub content: String,
}
#[derive(Clone)]
pub struct Prompt {
    pub boxes: Vec<PromptBox>,
    pub theme: PromptTheme,
}
#[derive(Clone, Default, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum PromptConnection {
    None,
    #[default]
    Line,
    Dot,
}
impl ToString for PromptConnection {
    fn to_string(&self) -> String {
        String::from(match self {
            Self::None => " ",
            Self::Line => "─",
            Self::Dot => "·",
        })
    }
}
struct PromptCurveLine {
    top_left: String,
    top_right: String,
    bottom_left: String,
    bottom_right: String,
}
impl Default for PromptCurveLine {
    fn default() -> Self {
        let top_left = "╭".to_string();
        let top_right = "╮".to_string();
        let bottom_left = "╰".to_string();
        let bottom_right = "╯".to_string();
        Self {
            top_left,
            top_right,
            bottom_left,
            bottom_right,
        }
    }
}
#[derive(Clone, Default, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum PromptSeparation {
    Block,
    #[default]
    Sharp,
    Slash,
    Round,
    Blur,
}
struct PromptSeparationBox {
    pub left: String,
    pub right: String,
}
struct PromptSeparationLine {
    pub left: String,
    pub right: String,
}
impl PromptSeparationBox {
    pub fn new(left: &str, right: &str) -> Self {
        Self {
            left: left.to_string(),
            right: right.to_string(),
        }
    }
}
impl PromptSeparationLine {
    pub fn new(left: &str, right: &str) -> Self {
        Self {
            left: left.to_string(),
            right: right.to_string(),
        }
    }
}
impl From<PromptSeparation> for PromptSeparationBox {
    fn from(value: PromptSeparation) -> Self {
        value.sep_box()
    }
}
impl From<PromptSeparation> for PromptSeparationLine {
    fn from(value: PromptSeparation) -> Self {
        value.sep_line()
    }
}
impl PromptSeparation {
    pub fn sep_box(&self) -> PromptSeparationBox {
        match self {
            Self::Slash => PromptSeparationBox::new("", ""),
            Self::Block => PromptSeparationBox::new(" ", " "),
            Self::Sharp => PromptSeparationBox::new("", ""),
            Self::Round => PromptSeparationBox::new("", ""),
            Self::Blur => PromptSeparationBox::new("▓▒░", "░▒▓"),
        }
    }
    pub fn sep_line(&self) -> PromptSeparationLine {
        match self {
            Self::Slash => PromptSeparationLine::new("╱", "╱"),
            Self::Block => PromptSeparationLine::new("|", "|"),
            Self::Sharp => PromptSeparationLine::new("", ""),
            Self::Round => PromptSeparationLine::new("", ""),
            Self::Blur => PromptSeparationLine::new("▓▒░", "░▒▓"),
        }
    }
}
