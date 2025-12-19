use zsh_seq::NamedColor;
use serde::{Deserialize, Serialize};
mod named_color_serde;
use crate::zsh::prompt::{PromptConnection, PromptSeparation};
use super::theme_manager;
use dialoguer::{Input, Select};
use std::str::FromStr;
use std::fmt;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct PromptTheme {
    pub color: PromptColorScheme,
    pub connection: PromptConnection,
    pub separation: PromptSeparation,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PromptColorScheme {
    #[serde(with = "named_color_serde")]
    pub bg: NamedColor,
    #[serde(with = "named_color_serde")]
    pub fg: NamedColor,
    pub separation: SeparationColor,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SeparationColor {
    Single(#[serde(with = "named_color_serde")] NamedColor),
    Rainbow(f32),
}
impl SeparationColor {
    pub fn get(&self, progress: f32) -> NamedColor {
        match self {
            Self::Single(color) => color.to_owned(),
            Self::Rainbow(start_hue) => {
                let hue = (start_hue + progress * 360.0) % 360.0;
                let rgb = hsl_to_rgb(hue, 1.0, 0.5);
                NamedColor::FullColor(rgb)
            }
        }
    }
}

/// HSLからRGB(u8, u8, u8)へ変換する補助関数
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r_prime, g_prime, b_prime) = if (0.0..60.0).contains(&h) {
        (c, x, 0.0)
    } else if (60.0..120.0).contains(&h) {
        (x, c, 0.0)
    } else if (120.0..180.0).contains(&h) {
        (0.0, c, x)
    } else if (180.0..240.0).contains(&h) {
        (0.0, x, c)
    } else {
        (c, 0.0, x)
    };

    (
        ((r_prime + m) * 255.0).round() as u8,
        ((g_prime + m) * 255.0).round() as u8,
        ((b_prime + m) * 255.0).round() as u8,
    )
}

impl Default for PromptColorScheme {
    fn default() -> Self {
        Self {
            bg: NamedColor::Black,
            fg: NamedColor::White,
            separation: SeparationColor::Single(NamedColor::LightBlack),
        }
    }
}


// Helper for PromptConnection FromStr
impl FromStr for PromptConnection {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "None" => Ok(PromptConnection::None),
            "Line" => Ok(PromptConnection::Line),
            "Dot" => Ok(PromptConnection::Dot),
            _ => Err(format!("Unknown PromptConnection: {}", s)),
        }
    }
}

// Helper for PromptSeparation FromStr
impl FromStr for PromptSeparation {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Block" => Ok(PromptSeparation::Block),
            "Sharp" => Ok(PromptSeparation::Sharp),
            "Slash" => Ok(PromptSeparation::Slash),
            "Round" => Ok(PromptSeparation::Round),
            "Blur" => Ok(PromptSeparation::Blur),
            _ => Err(format!("Unknown PromptSeparation: {}", s)),
        }
    }
}

fn prompt_for_named_color(prompt_text: &str, default_color: &NamedColor) -> NamedColor {
    Input::new()
        .with_prompt(prompt_text)
        .default(default_color.to_string())
        .interact_text()
        .map(|s| named_color_serde::deserialize_from_str(s.as_str()).unwrap_or_else(|e| {
            eprintln!("Invalid color input: {}. Using default.", e);
            default_color.clone()
        }))
        .unwrap_or_else(|e| {
            eprintln!("Error reading input: {}. Using default.", e);
            default_color.clone()
        })
}

fn configure_colors(theme: &mut PromptTheme) {
    println!("\n--- Configure Colors ---");

    theme.color.bg = prompt_for_named_color("Enter background color (e.g., Black, FullColor(255,0,0))", &theme.color.bg);
    theme.color.fg = prompt_for_named_color("Enter foreground color (e.g., White, Code256(123))", &theme.color.fg);

    let separation_type_options = ["Single Color", "Rainbow"];
    let selection = Select::new()
        .with_prompt("Choose separation color type")
        .items(&separation_type_options)
        .default(match theme.color.separation {
            SeparationColor::Single(_) => 0,
            SeparationColor::Rainbow(_) => 1,
        })
        .interact()
        .unwrap();

    theme.color.separation = match selection {
        0 => {
            let default_sep_color = match &theme.color.separation {
                SeparationColor::Single(c) => c.clone(),
                _ => NamedColor::LightBlack, // Default if converting from Rainbow
            };
            let color = prompt_for_named_color(
                "Enter single separation color (e.g., LightBlack)",
                &default_sep_color,
            );
            SeparationColor::Single(color)
        }
        1 => {
            let default_hue = match theme.color.separation {
                SeparationColor::Rainbow(h) => h,
                _ => 0.0, // Default if converting from Single
            };
            let hue = Input::new()
                .with_prompt("Enter rainbow start hue (0.0-360.0)")
                .default(default_hue)
                .interact_text()
                .unwrap_or(default_hue);
            SeparationColor::Rainbow(hue)
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
        .with_prompt("Choose prompt connection style")
        .items(&options.iter().map(|c| c.to_string()).collect::<Vec<String>>())
        .default(options.iter().position(|&p| p == theme.connection).unwrap_or(0))
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
        .with_prompt("Choose prompt separation style")
        .items(&options.iter().map(|s| format!("{:?}", s)).collect::<Vec<String>>())
        .default(options.iter().position(|&p| p == theme.separation).unwrap_or(0))
        .interact()
        .unwrap();

    theme.separation = options[selection];
}


pub fn main() {
    let mut current_theme = theme_manager::load_theme();

    loop {
        println!("\n--- Zsh Infinite Theme Configuration ---");
        println!("Current Theme: {:?}", current_theme);

        let options = ["Configure Colors", "Configure Connection", "Configure Separators", "Save and Exit"];
        let selection = Select::new()
            .with_prompt("What do you want to configure?")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => configure_colors(&mut current_theme),
            1 => configure_connection(&mut current_theme),
            2 => configure_separation(&mut current_theme),
            3 => {
                if let Err(e) = theme_manager::save_theme(&current_theme) {
                    eprintln!("Error saving theme: {}", e);
                }
                println!("Theme saved. Exiting configuration.");
                break;
            }
            _ => unreachable!(),
        }
    }
}