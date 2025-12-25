use crate::args::{PromptType, ZshCommands};

mod prompt;
pub mod theme; // Make theme public
mod theme_manager;
pub use theme_manager::{load_theme, save_theme};

pub async fn main(command: ZshCommands) {
    match command {
        ZshCommands::Prompt { side } => match side {
            PromptType::Left => prompt::left().await,
            PromptType::Right => prompt::right().await,
            PromptType::Hook => prompt::hook(),
            PromptType::Transient { exit_code } => prompt::transient(exit_code).await,
        },
        ZshCommands::BuildIn { segment } => {
            prompt::segment(segment);
        }
    }
}
