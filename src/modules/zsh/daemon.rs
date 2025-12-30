use crate::args::DaemonCommands;

mod get;
mod paths;
mod server;
mod start;
mod stop;
pub use get::get;
use start::start;
use stop::stop;

pub async fn main(command: DaemonCommands) {
    match command {
        DaemonCommands::Restart => {
            stop().await;
            start().await;
        }
        DaemonCommands::Start => {
            start().await;
        }
        DaemonCommands::Stop => {
            stop().await;
        }
    }
}
