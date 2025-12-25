mod hook;
mod left;
mod right;
mod segment;
mod transient;
pub use hook::hook;
pub use left::left;
pub use right::right;
pub use segment::segment;
use serde::{Deserialize, Serialize};
use std::fmt;
pub use transient::transient;
use zsh_seq::ZshPromptBuilder;

use crate::zsh::theme::prompt_theme::PromptContents;

impl Prompt {
    fn left_separation(&self) -> usize {
        if self.left.is_empty() {
            0
        } else {
            self.left.len() + 1
        }
    }
    fn right_separation(&self) -> usize {
        if self.right.is_empty() {
            0
        } else {
            self.right.len() + 1
        }
    }
    fn total_separation(&self) -> usize {
        self.left_separation() + self.right_separation()
    }
    pub fn add_left(&mut self, content: &str) {
        self.left.push(content.to_string());
    }
    pub fn add_right(&mut self, content: &str) {
        self.right.push(content.to_string());
    }
    fn render_left_fg(&self, prompt_contents: &PromptContents) -> ZshPromptBuilder {
        if self.left.is_empty() {
            return ZshPromptBuilder::new();
        }

        let color_scheme = &prompt_contents.color;
        let seps = &prompt_contents.left_segment_separators;
        let bg_color = color_scheme.bg;
        let total = (self.total_separation() + 1) as f32;

        let mut builder = ZshPromptBuilder::new();

        // 開始キャップ (edge_cap が有効なら Box を描画)
        if seps.edge_cap {
            let start_color = color_scheme.accent.get(0.0);
            builder = builder
                .color(start_color)
                .str(&seps.start_separator.sep_box().right)
                .end_color()
                .color_bg(start_color)
                .color(bg_color)
                .str(&seps.start_separator.sep_box().right)
                .end_color()
                .end_color_bg();
        } else {
            builder = builder
                .color(bg_color)
                .str(&seps.start_separator.sep_box().right)
                .end_color();
        }

        let len = self.left.len();
        for (i, content) in self.left.iter().enumerate() {
            builder = builder.color_bg(bg_color).str(content).end_color_bg();
            if i < len - 1 {
                let color_pos = (i + 1) as f32 / total;
                let sep_color = color_scheme.accent.get(color_pos);

                if seps.bold_separation {
                    // 太い区切りの場合は背景色を切り替えつつ Box を使用
                    builder = builder
                        .color(bg_color)
                        .color_bg(sep_color)
                        .str(&seps.mid_separator.sep_box().left)
                        .color_bg(bg_color)
                        .color(sep_color)
                        .str(&seps.mid_separator.sep_box().left)
                        .end_color()
                        .end_color_bg();
                } else {
                    // 通常は細線を使用
                    builder = builder
                        .color_bg(bg_color)
                        .color(sep_color)
                        .str(&seps.mid_separator.sep_line().left)
                        .end_color()
                        .end_color_bg();
                }
            }
        }

        // 終了キャップ
        if seps.edge_cap {
            let end_color = color_scheme
                .accent
                .get(self.left_separation() as f32 / total);
            builder = builder
                .color(bg_color)
                .color_bg(end_color)
                .str(&seps.end_separator.sep_box().left)
                .end_color_bg()
                .color(end_color)
                .str(&seps.end_separator.sep_box().left)
                .end_color();
        } else {
            builder = builder
                .color(bg_color)
                .end_color_bg()
                .str(&seps.end_separator.sep_box().left)
                .end_color();
        }

        builder
    }

    fn render_left_bg(&self, prompt_contents: &PromptContents) -> ZshPromptBuilder {
        if self.left.is_empty() {
            return ZshPromptBuilder::new();
        }

        let color_scheme = &prompt_contents.color;
        let seps = &prompt_contents.left_segment_separators;
        let bg_color = color_scheme.bg;
        let total = self.left.len() + self.right.len();
        let total = total as f32;

        let mut builder = ZshPromptBuilder::new();

        let start_color = color_scheme.accent.get(0.0);
        if seps.edge_cap {
            builder = builder
                .color(bg_color)
                .str(&seps.start_separator.sep_box().right)
                .end_color()
                .color_bg(bg_color)
                .color(start_color)
                .str(&seps.start_separator.sep_box().right)
                .end_color()
                .end_color_bg();
        } else {
            builder = builder
                .color(start_color)
                .str(&seps.start_separator.sep_box().right)
                .end_color();
        }

        let len = self.left.len();
        for (i, content) in self.left.iter().enumerate() {
            let color_pos = i as f32 / total;
            let sep_color = color_scheme.accent.get(color_pos);
            builder = builder.color_bg(sep_color).str(content).end_color_bg();
            if i < len - 1 {
                if seps.bold_separation {
                    let next_color_pos = (i + 1) as f32 / total;
                    let next_sep_color = color_scheme.accent.get(next_color_pos);
                    // 太い区切りの場合は背景色を切り替えつつ Box を使用
                    builder = builder
                        .color(sep_color)
                        .color_bg(bg_color)
                        .str(&seps.mid_separator.sep_box().left)
                        .color_bg(next_sep_color)
                        .color(bg_color)
                        .str(&seps.mid_separator.sep_box().left)
                        .end_color()
                        .end_color_bg();
                } else {
                    builder = builder
                        .color(sep_color)
                        .color_bg(bg_color)
                        .str(&seps.mid_separator.sep_box().left)
                        .end_color()
                        .end_color_bg();
                }
            }
        }

        // 終了キャップ
        let end_color = color_scheme
            .accent
            .get((self.left.len() - 1) as f32 / total);

        if seps.edge_cap {
            builder = builder
                .color(end_color)
                .color_bg(bg_color)
                .str(&seps.end_separator.sep_box().left)
                .end_color_bg()
                .color(bg_color)
                .str(&seps.end_separator.sep_box().left)
                .end_color();
        } else {
            builder = builder
                .color(end_color)
                .end_color_bg()
                .str(&seps.end_separator.sep_box().left)
                .end_color();
        }

        builder
    }
    pub fn render_right_fg(&self, prompt_contents: &PromptContents) -> ZshPromptBuilder {
        if self.right.is_empty() {
            return ZshPromptBuilder::new();
        }

        let color_scheme = &prompt_contents.color;
        let seps = &prompt_contents.right_segment_separators;
        let bg_color = color_scheme.bg;
        let total = (self.total_separation() + 1) as f32;
        let mut builder = ZshPromptBuilder::new();

        // 右側の開始キャップ
        if seps.edge_cap {
            let start_pos = (self.left_separation() + 1) as f32 / total;
            let start_color = color_scheme.accent.get(start_pos);
            builder = builder
                .color(start_color)
                .str(&seps.start_separator.sep_box().right)
                .end_color()
                .color_bg(start_color)
                .color(bg_color)
                .str(&seps.start_separator.sep_box().right)
                .end_color()
                .end_color_bg();
        } else {
            builder = builder
                .color(bg_color)
                .str(&seps.start_separator.sep_box().right)
                .end_color();
        }

        let len = self.right.len();
        for (i, content) in self.right.iter().enumerate() {
            builder = builder.color_bg(bg_color).str(content).end_color_bg();

            if i < len - 1 {
                let color_pos = (self.left_separation() + i + 2) as f32 / total;
                let sep_color = color_scheme.accent.get(color_pos);

                if seps.bold_separation {
                    builder = builder
                        .color(sep_color)
                        .color_bg(bg_color)
                        .str(&seps.mid_separator.sep_box().right)
                        .color(bg_color)
                        .color_bg(sep_color)
                        .str(&seps.mid_separator.sep_box().right)
                        .end_color()
                        .end_color_bg();
                } else {
                    builder = builder
                        .color_bg(bg_color)
                        .color(sep_color)
                        .str(&seps.mid_separator.sep_line().right)
                        .end_color()
                        .end_color_bg();
                }
            }
        }

        // 右端のキャップ
        if seps.edge_cap {
            let end_color = color_scheme.accent.get(1.0);
            builder = builder
                .color(bg_color)
                .color_bg(end_color)
                .str(&seps.end_separator.sep_box().left)
                .end_color_bg()
                .color(end_color)
                .str(&seps.end_separator.sep_box().left)
                .end_color();
        } else {
            builder = builder
                .color(bg_color)
                .end_color_bg()
                .str(&seps.end_separator.sep_box().left)
                .end_color();
        }

        builder
    }

    pub fn render_right_bg(&self, prompt_contents: &PromptContents) -> ZshPromptBuilder {
        if self.right.is_empty() {
            return ZshPromptBuilder::new();
        }

        let color_scheme = &prompt_contents.color;
        let seps = &prompt_contents.right_segment_separators;
        let bg_color = color_scheme.bg;
        let total = self.total_separation() as f32;
        let mut builder = ZshPromptBuilder::new();

        let start_pos = (self.left_separation() + 1) as f32 / total;
        let start_color = color_scheme.accent.get(start_pos);
        // 右側の開始キャップ
        if seps.edge_cap {
            builder = builder
                .color(bg_color)
                .str(&seps.start_separator.sep_box().right)
                .end_color()
                .color_bg(bg_color)
                .color(start_color)
                .str(&seps.start_separator.sep_box().right)
                .end_color()
                .end_color_bg();
        } else {
            builder = builder
                .color(start_color)
                .str(&seps.start_separator.sep_box().right)
                .end_color();
        }

        let len = self.right.len();
        for (i, content) in self.right.iter().enumerate() {
            let color_pos = (self.left_separation() + i + 1) as f32 / total;
            let sep_color = color_scheme.accent.get(color_pos);
            builder = builder.color_bg(sep_color).str(content).end_color_bg();
            if i < len - 1 {
                let next_color_pos = (self.left_separation() + i + 2) as f32 / total;
                let next_sep_color = color_scheme.accent.get(next_color_pos);
                if seps.bold_separation {
                    builder = builder
                        .color(bg_color)
                        .color_bg(sep_color)
                        .str(&seps.mid_separator.sep_box().right)
                        .color_bg(bg_color)
                        .color(next_sep_color)
                        .str(&seps.mid_separator.sep_box().right)
                        .end_color()
                        .end_color_bg();
                } else {
                    builder = builder
                        .color_bg(next_sep_color)
                        .color(bg_color)
                        .str(&seps.mid_separator.sep_line().right)
                        .end_color()
                        .end_color_bg();
                }
            }
        }

        // 右端のキャップ
        let end_color = color_scheme.accent.get(1.0 - 1.0 / total);
        if seps.edge_cap {
            builder = builder
                .color(end_color)
                .color_bg(bg_color)
                .str(&seps.end_separator.sep_box().left)
                .end_color_bg()
                .color(bg_color)
                .str(&seps.end_separator.sep_box().left)
                .end_color();
        } else {
            builder = builder
                .color(end_color)
                .end_color_bg()
                .str(&seps.end_separator.sep_box().left)
                .end_color();
        }

        builder
    }
    pub fn render_left(&self, prompt_contents: &PromptContents) -> ZshPromptBuilder {
        match prompt_contents.accent_which {
            crate::zsh::theme::prompt_theme::AccentWhich::ForeGround => {
                self.render_left_fg(prompt_contents)
            }
            crate::zsh::theme::prompt_theme::AccentWhich::BackGround => {
                self.render_left_bg(prompt_contents)
            }
        }
    }
    pub fn render_right(&self, prompt_contents: &PromptContents) -> ZshPromptBuilder {
        match prompt_contents.accent_which {
            crate::zsh::theme::prompt_theme::AccentWhich::ForeGround => {
                self.render_right_fg(prompt_contents)
            }
            crate::zsh::theme::prompt_theme::AccentWhich::BackGround => {
                self.render_right_bg(prompt_contents)
            }
        }
    }
}

#[derive(Clone, Default)]
pub struct Prompt {
    left: Vec<String>,
    right: Vec<String>,
}
#[derive(Clone, Default, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum PromptConnection {
    #[default]
    None, // 空白
    Line,     // 標準の細線 (─)
    Double,   // 二重線 (═)
    Bold,     // 太線 (━)
    Dashed,   // 破線 (╌)
    Dotted,   // 点線 (┄)
    Dot,      // 中点 (·)
    Bullet,   // 弾丸 (•)
    Wave,     // 波線 (〜)
    ZigZag,   // ギザギザ (≈)
    Bar,      // 太いバー (█)
    Gradient, // グラデーション (░▒▓)
}

impl fmt::Display for PromptConnection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::None => " ",
            Self::Line => "─",
            Self::Double => "═",
            Self::Bold => "━",
            Self::Dashed => "╌",
            Self::Dotted => "┄",
            Self::Dot => "·",
            Self::Bullet => "•",
            Self::Wave => "~",
            Self::ZigZag => "≈",
            Self::Bar => "█",
            Self::Gradient => "▒",
        };
        write!(f, "{}", s)
    }
}
struct PromptCurveLine {
    top_left: String,
    top_right: String,
    bottom_left: String,
    bottom_right: String,
    horizontal: String, // 横線 ─
    #[allow(unused)]
    vertical: String, // 縦線 │
    cross_left: String, // 縦線から右に枝分かれ ├
    cross_right: String,
}
impl Default for PromptCurveLine {
    fn default() -> Self {
        Self {
            top_left: "╭".to_string(),
            top_right: "╮".to_string(),
            bottom_left: "╰".to_string(),
            bottom_right: "╯".to_string(),
            horizontal: "─".to_string(),
            vertical: "│".to_string(),
            cross_left: "├".to_string(),
            cross_right: "┤".to_string(),
        }
    }
}
impl From<PromptConnection> for PromptCurveLine {
    fn from(conn: PromptConnection) -> Self {
        match conn {
            // 二重線
            PromptConnection::Double => Self {
                top_left: "╔".to_string(),
                top_right: "╗".to_string(),
                bottom_left: "╚".to_string(),
                bottom_right: "╝".to_string(),
                horizontal: "═".to_string(),
                vertical: "║".to_string(),
                cross_left: "╠".to_string(),
                cross_right: "╣".to_string(),
            },
            // 太線
            PromptConnection::Bold => Self {
                top_left: "┏".to_string(),
                top_right: "┓".to_string(),
                bottom_left: "┗".to_string(),
                bottom_right: "┛".to_string(),
                horizontal: "━".to_string(),
                vertical: "┃".to_string(),
                cross_left: "┣".to_string(),
                cross_right: "┫".to_string(),
            },
            // 標準の直角
            PromptConnection::Line | PromptConnection::Dashed | PromptConnection::Dotted => Self {
                top_left: "┌".to_string(),
                top_right: "┐".to_string(),
                bottom_left: "└".to_string(),
                bottom_right: "┘".to_string(),
                horizontal: "─".to_string(),
                vertical: "│".to_string(),
                cross_left: "├".to_string(),
                cross_right: "┤".to_string(),
            },
            // バー
            PromptConnection::Bar => Self {
                top_left: "".to_string(),
                top_right: "".to_string(),
                bottom_left: "".to_string(),
                bottom_right: "".to_string(),
                horizontal: "█".to_string(),
                vertical: "█".to_string(),
                cross_left: "█".to_string(),
                cross_right: "█".to_string(),
            },
            // 丸角（デフォルト）
            _ => Self {
                top_left: "╭".to_string(),
                top_right: "╮".to_string(),
                bottom_left: "╰".to_string(),
                bottom_right: "╯".to_string(),
                horizontal: conn.to_string(),
                vertical: "│".to_string(),
                cross_left: "├".to_string(),
                cross_right: "┤".to_string(),
            },
        }
    }
}

#[derive(Clone, Default, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum PromptSeparation {
    Block,
    #[default]
    Sharp, // 三角形 (Powerline Default)
    Slash,     // 斜線
    BackSlash, // 逆斜線
    Round,     // 半円
    Blur,      // グラデーション
    Flame,     // 炎
    Pixel,     // ドット/ピクセル
    Wave,      // 波形
    Lego,      // レゴブロック風
}
pub struct PromptSeparationBox {
    pub left: String,
    pub right: String,
}
pub struct PromptSeparationLine {
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
            Self::Block => PromptSeparationBox::new(" ", " "),
            Self::Sharp => PromptSeparationBox::new("", ""), // Powerline三角形
            Self::Slash => PromptSeparationBox::new("", ""), // 斜線
            Self::BackSlash => PromptSeparationBox::new("", ""), // 逆斜線
            Self::Round => PromptSeparationBox::new("", ""), // 半円
            Self::Blur => PromptSeparationBox::new("▓▒░", "░▒▓"), // グラデ
            Self::Flame => PromptSeparationBox::new("", ""), // 炎
            Self::Pixel => PromptSeparationBox::new("", ""), // ピクセル
            Self::Wave => PromptSeparationBox::new("", ""),  // 波
            Self::Lego => PromptSeparationBox::new("", ""),  // (代替)
        }
    }
    pub fn sep_line(&self) -> PromptSeparationLine {
        match self {
            Self::Block => PromptSeparationLine::new("|", "|"),
            Self::Sharp => PromptSeparationLine::new("", ""), // 細い三角形
            Self::Slash => PromptSeparationLine::new("╱", "╱"), // 細い斜線
            Self::BackSlash => PromptSeparationLine::new("╲", "╲"), // 細い逆斜線
            Self::Round => PromptSeparationLine::new("", ""), // 細い半円
            Self::Blur => PromptSeparationLine::new("░", "░"),  // 薄い網掛け
            Self::Flame => PromptSeparationLine::new("", ""), // 細い炎
            Self::Pixel => PromptSeparationLine::new("", ""), // 細いピクセル
            Self::Wave => PromptSeparationLine::new("", ""),  // 細い波
            Self::Lego => PromptSeparationLine::new("", ""),  // (代替)
        }
    }
}
