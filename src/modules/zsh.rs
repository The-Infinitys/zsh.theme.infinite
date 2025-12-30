use crate::args::{PromptType, ZshCommands};

pub mod daemon;
mod prompt;
pub mod theme;
pub use theme::manager::{load_theme, save_theme};
use zsh_seq::ZshPromptBuilder;

pub async fn main(command: ZshCommands) {
    match command {
        ZshCommands::Prompt { side } => {
            let builder = build_prompt(&side).await;
            print!("{}", builder.build())
        }
        ZshCommands::BuildIn { segment } => {
            prompt::segment(*segment);
        }
        ZshCommands::Daemon { command } => {
            daemon::main(command).await;
        }
    }
}
pub async fn build_prompt(prompt_type: &PromptType) -> ZshPromptBuilder {
    match prompt_type {
        PromptType::Left => prompt::left().await,
        PromptType::Right => prompt::right().await,
        PromptType::Hook => prompt::hook(),
        PromptType::Transient { exit_code } => prompt::transient(exit_code).await,
    }
}
