use clap::Parser;
use zsh_infinite::{args, utils, zsh};

#[tokio::main]
async fn main() {
    let args = args::Args::parse();
    match args.command {
        args::Commands::Zsh { command } => zsh::main(command).await,
        args::Commands::Update => utils::update(),
        args::Commands::Install => utils::install(),
        args::Commands::Uninstall => utils::uninstall(),
        #[cfg(debug_assertions)]
        args::Commands::Dev => utils::dev(),
        args::Commands::Theme { command } => {
            if let Some(command) = command {
                zsh::theme::set(command)
            } else {
                zsh::theme::main().await
            }
        }
    }
}
