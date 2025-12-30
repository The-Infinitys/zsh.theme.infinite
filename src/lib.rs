#![feature(trait_alias)]
mod modules;
pub use modules::*;
use once_cell::sync::Lazy;
mod zmod;
use crate::modules::zsh::theme::{self, prompt_theme::PromptTheme};
use zmod::setup;

static PROMPT_THEME: Lazy<PromptTheme> = Lazy::new(theme::manager::load_theme);

fn prompt_theme() -> &'static PromptTheme {
    &PROMPT_THEME
}

zsh_module::export_module!(zsh_infinite, setup);
