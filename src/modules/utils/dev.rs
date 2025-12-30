use crate::zsh::theme::prompt_theme::PromptTheme;
use std::{env, fs, process::Command};

pub fn dev() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let run_dir = current_dir.join("run");

    // --- 1. ディレクトリのクリーンアップと作成 ---
    if run_dir.exists() {
        fs::remove_dir_all(&run_dir).expect("Failed to clear run directory");
    }

    // 設定ファイルの配置先: run/.config/zsh-infinite/
    let config_dir = run_dir.join(".config").join("zsh-infinite");
    fs::create_dir_all(&config_dir).expect("Failed to create config directory");

    // --- 2. 自分自身のバイナリをコピー ---
    let exe_path = env::current_exe().expect("Failed to get current executable path");
    let exe_name = exe_path.file_name().expect("Failed to get file name");
    let target_path = run_dir.join(exe_name);
    fs::copy(&exe_path, &target_path).expect("Failed to copy self");

    // オブジェクトを生成
    let theme = PromptTheme::infinite();

    // YAMLにシリアライズ
    let theme_content = serde_yaml::to_string(&theme).expect("Failed to serialize theme");
    let theme_path = config_dir.join("theme.yaml");

    // 必要であればプレースホルダ置換（今回は直接生成しているので基本不要ですが、パス解決用など）
    let theme_content = theme_content.replace("{{RUN_DIR}}", &run_dir.to_string_lossy());
    fs::write(&theme_path, theme_content).expect("Failed to write theme.yaml");

    // --- 4. Zsh用設定ファイルの設置 (.zshrc, .zsh-theme) ---
    let zshrc_template = include_str!("../../assets/scripts/dev.zshrc");
    let zshrc_path = run_dir.join(".zshrc");
    let zshrc_content = zshrc_template.replace("{{RUN_DIR}}", &run_dir.to_string_lossy());
    fs::write(&zshrc_path, zshrc_content).expect("Failed to write .zshrc");

    let zsh_theme = include_str!("../../assets/scripts/infinite.zsh-theme");
    let zsh_theme_path = run_dir.join(".zsh-theme");
    fs::write(&zsh_theme_path, zsh_theme).expect("Failed to write infinite.zsh-theme");

    // --- 5. Zshの起動 ---
    println!("Starting clean zsh session (Dev mode)...");
    println!("Config path: {:?}", theme_path);

    let mut child = Command::new("zsh");

    child
        .env_clear()
        .env("HOME", &run_dir)
        .env("ZDOTDIR", &run_dir)
        .env("XDG_CONFIG_HOME", run_dir.join(".config"))
        .env("PATH", env::var("PATH").unwrap_or_default())
        .env("TERM", env::var("TERM").unwrap_or_default())
        .env("USER", env::var("USER").unwrap_or_default())
        .env("SHELL", "/bin/zsh")
        .current_dir(&run_dir);

    child
        .spawn()
        .expect("Failed to start zsh session")
        .wait()
        .expect("Failed to wait for shell session");
}
