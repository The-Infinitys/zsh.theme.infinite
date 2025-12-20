use clap::{Parser, Subcommand};
use std::cmp::Ordering;

#[derive(Parser)]
#[command(name = "Zsh Infinite", version, about = "CLI tool with nested subcommands")]
pub struct Args {
    /// Show verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// zsh related commands
    #[command(hide = true)]
    Zsh {
        #[command(subcommand)]
        command: ZshCommands,
    },
    /// Update the application
    Update,
    /// Install a component
    Install,
    /// Uninstall a component
    Uninstall,
    /// debug theme
    #[cfg(debug_assertions)]
    Dev,
    /// Manage Zsh theme
    Theme,
}

#[derive(Subcommand)]
pub enum ZshCommands {
    /// Prompt related commands
    Prompt {
        #[command(subcommand)]
        side: PromptType,
    },
}

#[derive(Subcommand, Clone, Copy, PartialEq, Eq, Debug)] // PartialOrd, Ord は手動実装
pub enum PromptType {
    Left,
    Right,
    Transient {
        #[arg(long, short = 'e')]
        exit_code: Option<i32>,
    },
}

impl PromptType {
    fn weight(&self) -> u8 {
        match self {
            Self::Left => 0,
            Self::Right => 1,
            Self::Transient { .. } => 2,
        }
    }
}

impl PartialOrd for PromptType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PromptType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight().cmp(&other.weight())
    }
}
