use crate::{args, utils, zsh};
use clap::Parser;
use tokio::runtime::Runtime;
use zsh_module::{Builtin, MaybeError, Module, ModuleBuilder, Opts};
struct ZshInfinite;

impl ZshInfinite {
    fn infinite_cmd(&mut self, _name: &str, args: &[&str], _opts: Opts) -> MaybeError {
        // Zshから渡された引数を clap で解析できるように変換
        // args[0] に相当するプログラム名が必要なため "zsh-infinite" を追加
        let mut clap_args = vec!["zsh-infinite"];
        clap_args.extend_from_slice(args);

        // 引数のパース（失敗時は clap がエラーを表示して終了しようとするため注意）
        let args = match args::Args::try_parse_from(clap_args.as_slice()) {
            Ok(a) => a,
            Err(e) => {
                eprintln!("{}", e);
                return Ok(()); // Zsh自体を落とさないよう Ok で返すか適切に処理
            }
        };

        let rt = Runtime::new().map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        // メインロジックの実行
        rt.block_on(async {
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
        });

        Ok(())
    }
}

pub fn setup() -> Result<Module, Box<dyn std::error::Error>> {
    let module = ModuleBuilder::new(ZshInfinite)
        .builtin(ZshInfinite::infinite_cmd, Builtin::new("zsh-infinite"))
        .build();
    Ok(module)
}
