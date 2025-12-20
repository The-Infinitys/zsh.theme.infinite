use serde::{Deserialize, Serialize};
use zsh_seq::NamedColor;

use super::gradient::{GradientPart, deserialize_gradient, serialize_gradient};
use super::named_color_serde;

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
    pub accent: AccentColor,
    pub accent_which: AccentWhich,
}
#[derive(Clone, Debug, Serialize, Deserialize, Default, Copy)]
pub enum AccentWhich {
    #[default]
    ForeGround,
    BackGround,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AccentColor {
    Single(#[serde(with = "named_color_serde")] NamedColor),
    Rainbow(#[serde(with = "named_color_serde")] NamedColor),
    #[serde(
        serialize_with = "serialize_gradient",
        deserialize_with = "deserialize_gradient"
    )]
    Gradient(GradientPart),
}

impl AccentColor {
    pub fn get(&self, progress: f32) -> NamedColor {
        match self {
            Self::Single(color) => color.to_owned(),
            Self::Rainbow(base_color) => {
                let (r, g, b) = match base_color {
                    NamedColor::FullColor(rgb) => *rgb,
                    _ => (255, 0, 0), // デフォルト赤
                };
                let (start_hue, saturation, lightness) = super::gradient::rgb_to_hsl(r, g, b);
                let hue = (start_hue + progress * 360.0) % 360.0;
                let rgb = super::gradient::hsl_to_rgb(hue, saturation, lightness);
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
                        return NamedColor::FullColor(super::gradient::lerp_rgb_color(
                            *c1_rgb, *c2_rgb, t,
                        ));
                    }
                }
                NamedColor::FullColor(stops[0].0)
            }
        }
    }
}

impl Default for PromptColorScheme {
    fn default() -> Self {
        Self {
            bg: NamedColor::Black,
            fg: NamedColor::White,
            pc: NamedColor::Cyan,
            sc: NamedColor::LightBlack,
            accent: AccentColor::Single(NamedColor::LightBlack),
            accent_which: AccentWhich::default(),
        }
    }
}
