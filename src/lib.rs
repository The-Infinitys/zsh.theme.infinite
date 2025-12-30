mod modules;
pub use modules::*;
use once_cell::sync::Lazy;

use crate::modules::zsh::theme::{self, prompt_theme::PromptTheme};
mod build_in;

static PROMPT_THEME: Lazy<PromptTheme> = Lazy::new(theme::manager::load_theme);

fn prompt_theme() -> &'static PromptTheme {
    &PROMPT_THEME
}
