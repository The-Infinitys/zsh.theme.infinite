use crate::args::{PromptSide, ZshCommands};

mod prompt;
pub mod theme; // Make theme public
mod theme_manager;
pub use theme_manager::{load_theme, save_theme};

pub fn main(command: ZshCommands) {
    match command {
        ZshCommands::Prompt { side } => match side {
            PromptSide::Left => prompt::left(),
            PromptSide::Right => prompt::right(),
        },
    }
}
