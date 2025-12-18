use crate::args::{PromptSide, ZshCommands};

mod prompt;
mod theme;

pub fn main(command: ZshCommands) {
    match command {
        ZshCommands::Prompt { side } => match side {
            PromptSide::Left => prompt::left(),
            PromptSide::Right => prompt::right(),
        },
    }
}
