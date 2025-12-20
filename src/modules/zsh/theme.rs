use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use zsh_seq::NamedColor;
mod named_color_serde; // 既存のファイルをそのまま使用
use super::theme_manager;
use crate::zsh::prompt::{PromptConnection, PromptSeparation};
use dialoguer::{Input, Select};
use std::fmt;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct PromptTheme {
    pub color: PromptColorScheme,
    pub connection: PromptConnection,
    pub separation: PromptSeparation,
    pub prompt_contents: PromptContents,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PromptContents {
    pub left: Vec<PromptContent>,
    pub right: Vec<PromptContent>,
}

impl Default for PromptContents {
    fn default() -> Self {
        Self {
            left: vec![
                PromptContent::Shell(vec![
                    "zsh".to_string(),
                    "-c".to_string(),
                    "whoami".to_string(),
                ]),
                PromptContent::Shell(vec![
                    "zsh".to_string(),
                    "-c".to_string(),
                    "hostname".to_string(),
                ]),
            ],
            right: vec![
                PromptContent::Shell(vec![
                    "zsh".to_string(),
                    "-c".to_string(),
                    "echo ${PWD/#$HOME/\\~}".to_string(),
                ]),
                PromptContent::Env("?".to_string()),
            ],
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PromptContent {
    Shell(Vec<String>),
    Env(String),
}

use tokio::process::Command;

// ... 既存のコード ...

impl PromptContent {
    pub async fn content(&self) -> Option<String> {
        match self {
            Self::Shell(args) => {
                if args.is_empty() {
                    return None;
                }
                let output = Command::new(&args[0])
                    .args(&args[1..])
                    .output()
                    .await
                    .ok()?;

                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if stdout.is_empty() {
                        None
                    } else {
                        Some(stdout)
                    }
                } else {
                    None
                }
            }
            Self::Env(var_name) => std::env::var(var_name).ok(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PromptColorScheme {
    #[serde(with = "named_color_serde")]
    pub bg: NamedColor,
    #[serde(with = "named_color_serde")]
    pub fg: NamedColor,
    #[serde(with = "named_color_serde")]
    pub pc: NamedColor,
    #[serde(with = "named_color_serde")]
    pub sc: NamedColor,
    pub separation: SeparationColor,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SeparationColor {
    Single(#[serde(with = "named_color_serde")] NamedColor),
    // Rainbowをf32ではなくNamedColorで保持するように変更
    Rainbow(#[serde(with = "named_color_serde")] NamedColor),
    // Gradientにカスタムシリアライザを適用
    #[serde(
        serialize_with = "serialize_gradient",
        deserialize_with = "deserialize_gradient"
    )]
    Gradient(Vec<(NamedColor, f32)>),
}

// --- Gradient用のカスタムシリアライズ/デシリアライズ ---

fn serialize_gradient<S>(stops: &Vec<(NamedColor, f32)>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    use serde::ser::SerializeSeq;
    let mut seq = serializer.serialize_seq(Some(stops.len()))?;
    for (color, pos) in stops {
        // 既存の named_color_serde を利用して文字列化
        // 内部で serialize_to_str が提供されている前提、もしくは一時的なラッパーを使用
        let color_str = format!("{:?}", color); // ここは環境に合わせて調整してください
        seq.serialize_element(&format!("{}:{}", color_str, pos))?;
    }
    seq.end()
}

fn deserialize_gradient<'de, D>(deserializer: D) -> Result<Vec<(NamedColor, f32)>, D::Error>
where
    D: Deserializer<'de>,
{
    struct GradientVisitor;
    impl<'de> Visitor<'de> for GradientVisitor {
        type Value = Vec<(NamedColor, f32)>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a sequence of 'color:stop' strings")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: de::SeqAccess<'de>,
        {
            let mut stops = Vec::new();
            while let Some(s) = seq.next_element::<String>()? {
                let parts: Vec<&str> = s.split(':').collect();
                if parts.len() == 2 {
                    let color = named_color_serde::deserialize_from_str(parts[0])
                        .map_err(de::Error::custom)?;
                    let pos = parts[1].parse::<f32>().map_err(de::Error::custom)?;
                    stops.push((color, pos));
                }
            }
            Ok(stops)
        }
    }
    deserializer.deserialize_seq(GradientVisitor)
}

// --- 色計算ロジック ---

impl SeparationColor {
    pub fn get(&self, progress: f32) -> NamedColor {
        match self {
            Self::Single(color) => color.to_owned(),
            Self::Rainbow(base_color) => {
                let (r, g, b) = match base_color {
                    NamedColor::FullColor(rgb) => *rgb,
                    _ => (255, 0, 0), // デフォルト赤
                };
                let (start_hue, saturation, lightness) = rgb_to_hsl(r, g, b);
                let hue = (start_hue + progress * 360.0) % 360.0;
                let rgb = hsl_to_rgb(hue, saturation, lightness);
                NamedColor::FullColor(rgb)
            }
            Self::Gradient(stops) => {
                if stops.is_empty() {
                    return NamedColor::White;
                }
                if stops.len() == 1 {
                    return stops[0].0;
                }

                let mut sorted_stops = stops.clone();
                sorted_stops.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

                let first = &sorted_stops[0];
                if progress <= first.1 {
                    return first.0;
                }
                let last = &sorted_stops[sorted_stops.len() - 1];
                if progress >= last.1 {
                    return last.0;
                }

                for i in 0..sorted_stops.len() - 1 {
                    let (c1, p1) = &sorted_stops[i];
                    let (c2, p2) = &sorted_stops[i + 1];
                    if progress >= *p1 && progress <= *p2 {
                        let t = (progress - p1) / (p2 - p1);
                        return lerp_named_color(c1, c2, t);
                    }
                }
                stops[0].0
            }
        }
    }
}

fn lerp_named_color(c1: &NamedColor, c2: &NamedColor, t: f32) -> NamedColor {
    let get_rgb = |c: &NamedColor| match c {
        NamedColor::FullColor(rgb) => *rgb,
        _ => (128, 128, 128),
    };
    let rgb1 = get_rgb(c1);
    let rgb2 = get_rgb(c2);

    let r = (rgb1.0 as f32 + (rgb2.0 as f32 - rgb1.0 as f32) * t) as u8;
    let g = (rgb1.1 as f32 + (rgb2.1 as f32 - rgb1.1 as f32) * t) as u8;
    let b = (rgb1.2 as f32 + (rgb2.2 as f32 - rgb1.2 as f32) * t) as u8;
    NamedColor::FullColor((r, g, b))
}

fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let r_f = r as f32 / 255.0;
    let g_f = g as f32 / 255.0;
    let b_f = b as f32 / 255.0;

    let max = r_f.max(g_f).max(b_f);
    let min = r_f.min(g_f).min(b_f);
    let delta = max - min;

    let mut h = 0.0;
    let s = if max == 0.0 {
        0.0
    } else {
        delta / (1.0 - (2.0 * ((max + min) / 2.0) - 1.0).abs())
    };
    let l = (max + min) / 2.0;

    if delta != 0.0 {
        h = if max == r_f {
            (g_f - b_f) / delta % 6.0
        } else if max == g_f {
            (b_f - r_f) / delta + 2.0
        } else {
            (r_f - g_f) / delta + 4.0
        };
        h *= 60.0;
        if h < 0.0 {
            h += 360.0;
        }
    }
    (h, s, l)
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;
    let (r_p, g_p, b_p) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    (
        ((r_p + m) * 255.0) as u8,
        ((g_p + m) * 255.0) as u8,
        ((b_p + m) * 255.0) as u8,
    )
}

impl Default for PromptColorScheme {
    fn default() -> Self {
        Self {
            bg: NamedColor::Black,
            fg: NamedColor::White,
            pc: NamedColor::Cyan,
            sc: NamedColor::LightBlack,
            separation: SeparationColor::Single(NamedColor::LightBlack),
        }
    }
}

// DisplayNamedColor
struct DisplayNamedColor<'a>(&'a NamedColor);
impl<'a> fmt::Display for DisplayNamedColor<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // named_color_serde内のデシリアライザが期待する形式に合わせる
        let s = match self.0 {
            NamedColor::Code256(c) => format!("Code256({})", c),
            NamedColor::FullColor((r, g, b)) => format!("#{:02X}{:02X}{:02X}", r, g, b),
            _ => format!("{:?}", self.0),
        };
        write!(f, "{}", s)
    }
}

fn prompt_for_named_color(prompt_text: &str, default_color: &NamedColor) -> NamedColor {
    Input::new()
        .with_prompt(prompt_text)
        .default(DisplayNamedColor(default_color).to_string())
        .interact_text()
        .map(|s| {
            named_color_serde::deserialize_from_str(&s).unwrap_or(*default_color)
        })
        .unwrap_or_else(|_| *default_color)
}

fn configure_colors(theme: &mut PromptTheme) {
    println!("\n--- Configure Colors ---");

    theme.color.bg = prompt_for_named_color("Background color", &theme.color.bg);
    theme.color.fg = prompt_for_named_color("Foreground color", &theme.color.fg);
    theme.color.pc = prompt_for_named_color("Primary color (pc)", &theme.color.pc);
    theme.color.sc = prompt_for_named_color("Secondary color (sc)", &theme.color.sc);

    let options = ["Single Color", "Rainbow", "Gradient"];
    let selection = Select::new()
        .with_prompt("Choose separation color type")
        .items(options)
        .default(match theme.color.separation {
            SeparationColor::Single(_) => 0,
            SeparationColor::Rainbow(_) => 1,
            SeparationColor::Gradient(_) => 2,
        })
        .interact()
        .unwrap();

    theme.color.separation = match selection {
        0 => SeparationColor::Single(prompt_for_named_color("Color", &NamedColor::LightBlack)),
        1 => {
            let color = prompt_for_named_color(
                "Rainbow Start Color (Hex)",
                &NamedColor::FullColor((255, 0, 0)),
            );
            SeparationColor::Rainbow(color)
        }
        2 => {
            let c1 = prompt_for_named_color("Start Color", &NamedColor::Cyan);
            let c2 = prompt_for_named_color("End Color", &NamedColor::Blue);
            SeparationColor::Gradient(vec![(c1, 0.0), (c2, 1.0)])
        }
        _ => unreachable!(),
    };
}

fn configure_connection(theme: &mut PromptTheme) {
    println!("\n--- Configure Connection ---");
    let options = [
        PromptConnection::None,
        PromptConnection::Line,
        PromptConnection::Dot,
    ];
    let selection = Select::new()
        .with_prompt("Choose style")
        .items(
            options
                .iter()
                .map(|o| format!("{:?}", o))
                .collect::<Vec<_>>(),
        )
        .default(
            options
                .iter()
                .position(|&p| p == theme.connection)
                .unwrap_or(0),
        )
        .interact()
        .unwrap();
    theme.connection = options[selection];
}

fn configure_separation(theme: &mut PromptTheme) {
    println!("\n--- Configure Separators ---");
    let options = [
        PromptSeparation::Block,
        PromptSeparation::Sharp,
        PromptSeparation::Slash,
        PromptSeparation::Round,
        PromptSeparation::Blur,
    ];
    let selection = Select::new()
        .with_prompt("Choose style")
        .items(
            options
                .iter()
                .map(|o| format!("{:?}", o))
                .collect::<Vec<_>>(),
        )
        .default(
            options
                .iter()
                .position(|&p| p == theme.separation)
                .unwrap_or(0),
        )
        .interact()
        .unwrap();
    theme.separation = options[selection];
}

pub async fn main() {
    let mut current_theme = theme_manager::load_theme();
    loop {
        println!("\n--- Zsh Infinite Theme Configuration ---");
        let options = [
            "Configure Colors",
            "Configure Connection",
            "Configure Separators",
            "Save and Exit",
        ];
        let selection = Select::new()
            .with_prompt("Main Menu")
            .items(options)
            .interact()
            .unwrap();

        match selection {
            0 => configure_colors(&mut current_theme),
            1 => configure_connection(&mut current_theme),
            2 => configure_separation(&mut current_theme),
            3 => {
                let _ = theme_manager::save_theme(&current_theme);
                break;
            }
            _ => unreachable!(),
        }
    }
}
