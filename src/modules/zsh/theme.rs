use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use zsh_seq::NamedColor;
mod named_color_serde; // 既存のファイルをそのまま使用
use super::theme_manager;
use crate::zsh::prompt::{PromptConnection, PromptSeparation};
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Select};
use std::fmt;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PromptTheme {
    pub color: PromptColorScheme,
    pub connection: PromptConnection,
    pub separation: PromptSeparation,
    pub prompt_contents_list: Vec<PromptContents>,
}
impl Default for PromptTheme {
    fn default() -> Self {
        Self {
            color: PromptColorScheme::default(),
            connection: PromptConnection::default(),
            separation: PromptSeparation::default(),
            prompt_contents_list: vec![PromptContents::default()],
        }
    }
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
                PromptContent::new(vec![
                    "zsh".to_string(),
                    "-c".to_string(),
                    "whoami".to_string(),
                ]),
                PromptContent::new(vec![
                    "zsh".to_string(),
                    "-c".to_string(),
                    "hostname".to_string(),
                ]),
            ],
            right: vec![
                PromptContent::new(vec![
                    "zsh".to_string(),
                    "-c".to_string(),
                    "echo ${PWD/#$HOME/\\~}".to_string(),
                ]),
                PromptContent::new(vec![
                    "zsh".to_string(),
                    "-c".to_string(),
                    "echo $?".to_string(),
                ]),
            ],
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PromptContent {
    shell: Vec<String>,
}

use tokio::process::Command;

impl PromptContent {
    pub fn new(shell: Vec<String>) -> Self {
        Self { shell }
    }
    pub async fn content(&self) -> Option<String> {
        if self.shell.is_empty() {
            return None;
        }
        let output = Command::new(&self.shell[0])
            .args(&self.shell[1..])
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
    Rainbow(#[serde(with = "named_color_serde")] NamedColor),
    // Gradientにカスタムシリアライザを適用
    #[serde(
        serialize_with = "serialize_gradient",
        deserialize_with = "deserialize_gradient"
    )]
    Gradient(Vec<((u8, u8, u8), f32)>),
}

// --- Gradient用のカスタムシリアライズ/デシリアライズ ---

fn serialize_gradient<S>(stops: &Vec<((u8, u8, u8), f32)>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    use serde::ser::SerializeSeq;
    let mut seq = serializer.serialize_seq(Some(stops.len()))?;
    for (rgb, pos) in stops {
        // フルカラー形式をHex文字列に変換して保存
        seq.serialize_element(&format!("#{:02X}{:02X}{:02X}:{}", rgb.0, rgb.1, rgb.2, pos))?;
    }
    seq.end()
}

fn deserialize_gradient<'de, D>(deserializer: D) -> Result<Vec<((u8, u8, u8), f32)>, D::Error>
where
    D: Deserializer<'de>,
{
    struct GradientVisitor;
    impl<'de> Visitor<'de> for GradientVisitor {
        type Value = Vec<((u8, u8, u8), f32)>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a sequence of '#RRGGBB:stop' strings")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: de::SeqAccess<'de>,
        {
            let mut stops = Vec::new();
            while let Some(s) = seq.next_element::<String>()? {
                let parts: Vec<&str> = s.split(':').collect();
                if parts.len() == 2 && parts[0].starts_with('#') && parts[0].len() == 7 {
                    let r = u8::from_str_radix(&parts[0][1..3], 16).map_err(de::Error::custom)?;
                    let g = u8::from_str_radix(&parts[0][3..5], 16).map_err(de::Error::custom)?;
                    let b = u8::from_str_radix(&parts[0][5..7], 16).map_err(de::Error::custom)?;
                    let pos = parts[1].parse::<f32>().map_err(de::Error::custom)?;
                    stops.push(((r, g, b), pos));
                } else {
                    return Err(de::Error::custom(format!(
                        "Invalid gradient stop format: {}",
                        s
                    )));
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
                    return NamedColor::FullColor(stops[0].0);
                }

                let mut sorted_stops = stops.clone();
                sorted_stops.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

                let first = &sorted_stops[0];
                if progress <= first.1 {
                    return NamedColor::FullColor(first.0);
                }
                let last = &sorted_stops[sorted_stops.len() - 1];
                if progress >= last.1 {
                    return NamedColor::FullColor(last.0);
                }

                for i in 0..sorted_stops.len() - 1 {
                    let (c1_rgb, p1) = &sorted_stops[i];
                    let (c2_rgb, p2) = &sorted_stops[i + 1];
                    if progress >= *p1 && progress <= *p2 {
                        let t = (progress - p1) / (p2 - p1);
                        return NamedColor::FullColor(lerp_rgb_color(*c1_rgb, *c2_rgb, t));
                    }
                }
                NamedColor::FullColor(stops[0].0)
            }
        }
    }
}

fn lerp_rgb_color(rgb1: (u8, u8, u8), rgb2: (u8, u8, u8), t: f32) -> (u8, u8, u8) {
    let r = (rgb1.0 as f32 + (rgb2.0 as f32 - rgb1.0 as f32) * t) as u8;
    let g = (rgb1.1 as f32 + (rgb2.1 as f32 - rgb1.1 as f32) * t) as u8;
    let b = (rgb1.2 as f32 + (rgb2.2 as f32 - rgb1.2 as f32) * t) as u8;
    (r, g, b)
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

fn create_default_rainbow_gradient() -> Vec<((u8, u8, u8), f32)> {
    vec![
        ((255, 0, 0), 0.0),    // Red
        ((255, 127, 0), 0.16), // Orange
        ((255, 255, 0), 0.32), // Yellow
        ((0, 255, 0), 0.48),   // Green
        ((0, 0, 255), 0.64),   // Blue
        ((75, 0, 130), 0.80),  // Indigo
        ((148, 0, 211), 1.0),  // Violet
    ]
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
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt_text)
        .default(DisplayNamedColor(default_color).to_string())
        .interact_text()
        .map(|s| named_color_serde::deserialize_from_str(&s).unwrap_or(*default_color))
        .unwrap_or_else(|_| *default_color)
}

// 新しい関数: フルカラーのRGB値をプロンプトで取得
fn prompt_for_rgb_color(prompt_text: &str, default_rgb: (u8, u8, u8)) -> (u8, u8, u8) {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt_text)
        .default(format!(
            "#{:02X}{:02X}{:02X}",
            default_rgb.0, default_rgb.1, default_rgb.2
        ))
        .interact_text()
        .map(|s| {
            if s.starts_with('#') && s.len() == 7 {
                if let (Ok(r), Ok(g), Ok(b)) = (
                    u8::from_str_radix(&s[1..3], 16),
                    u8::from_str_radix(&s[3..5], 16),
                    u8::from_str_radix(&s[5..7], 16),
                ) {
                    return (r, g, b);
                }
            }
            default_rgb
        })
        .unwrap_or(default_rgb)
}

fn configure_colors(theme: &mut PromptTheme) {
    println!("\n--- Configure Colors ---");

    theme.color.bg = prompt_for_named_color("Background color", &theme.color.bg);
    theme.color.fg = prompt_for_named_color("Foreground color", &theme.color.fg);
    theme.color.pc = prompt_for_named_color("Primary color (pc)", &theme.color.pc);
    theme.color.sc = prompt_for_named_color("Secondary color (sc)", &theme.color.sc);

    let options = [
        "Single Color",
        "Rainbow",
        "Default Rainbow Gradient",
        "Custom Gradient",
    ];
    let default_selection = match &theme.color.separation {
        SeparationColor::Single(_) => 0,
        SeparationColor::Rainbow(_) => 1,
        // Default Rainbow Gradient と Custom Gradient を区別するために、既存のグラデーションが
        // デフォルトの虹色グラデーションと一致するかどうかを簡易的に判定するか、
        // または単に Custom Gradient にフォールバックさせるかを検討。
        // ここでは簡単に Custom Gradient にフォールバックさせます。
        SeparationColor::Gradient(_) => 3, // Custom Gradient に対応
    };

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose separation color type")
        .items(options)
        .default(default_selection)
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
            // Default Rainbow Gradient
            SeparationColor::Gradient(create_default_rainbow_gradient())
        }
        3 => {
            // Custom Gradient (Existing 2-point gradient)
            let c1_rgb = prompt_for_rgb_color("Gradient Start Color (Hex)", (0, 255, 255)); // Cyan
            let c2_rgb = prompt_for_rgb_color("Gradient End Color (Hex)", (0, 0, 255)); // Blue
            SeparationColor::Gradient(vec![(c1_rgb, 0.0), (c2_rgb, 1.0)])
        }
        _ => unreachable!(),
    };
}

fn configure_connection(theme: &mut PromptTheme) {
    println!("\n--- Configure Connection ---");
    let options = [
        PromptConnection::None,
        PromptConnection::Line,
        PromptConnection::Double,
        PromptConnection::Bold,
        PromptConnection::Dashed,
        PromptConnection::Dotted,
        PromptConnection::Dot,
        PromptConnection::Bullet,
        PromptConnection::Wave,
        PromptConnection::ZigZag,
        PromptConnection::Bar,
        PromptConnection::Gradient,
    ];
    let selection = Select::with_theme(&ColorfulTheme::default())
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
        PromptSeparation::BackSlash,
        PromptSeparation::Round,
        PromptSeparation::Blur,
        PromptSeparation::Flame,
        PromptSeparation::Pixel,
        PromptSeparation::Wave,
        PromptSeparation::Lego,
    ];
    let selection = Select::with_theme(&ColorfulTheme::default())
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
        let selection = Select::with_theme(&ColorfulTheme::default())
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
