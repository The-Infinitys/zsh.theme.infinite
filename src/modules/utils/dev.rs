use std::{env, fs, process::Command};

pub fn dev() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let run_dir = current_dir.join("run");

    // --- 1. ディレクトリ作成 ---
    if !run_dir.exists() {
        fs::create_dir(&run_dir).expect("Failed to create run directory");
    }

    // --- 2. 自分自身のバイナリをコピー ---
    let exe_path = env::current_exe().expect("Failed to get current executable path");
    let target_path = run_dir.join(exe_path.file_name().expect("Failed to get file name"));
    fs::copy(&exe_path, &target_path).expect("Failed to copy self");

    // --- 3. 設定ファイルの作成 ---
    let zshrc_path = run_dir.join(".zshrc");
    let zshrc_content = include_str!("../../assets/scripts/dev.zshrc")
        .replace("{{RUN_DIR}}", &run_dir.to_string_lossy());
    let theme_path = run_dir.join(".zshrc");
    let theme_content = include_str!("../../assets/scripts/dev.zsh-theme")
        .replace("{{RUN_DIR}}", &run_dir.to_string_lossy());

    fs::write(&zshrc_path, zshrc_content).expect("Failed to write .zshrc");
    fs::write(&theme_path, theme_content).expect("Failed to write .zsh-theme");

    // --- 4. 環境変数をリセットしてZshを起動 ---
    println!("Starting clean zsh session...");

    let mut child = Command::new("zsh");

    child
        .env_clear() // ← これですべての環境変数を削除
        // 以下の変数は、Zshを正常に動かすために最低限必要
        .env("ZDOTDIR", &run_dir)
        .env("HOME", &run_dir)
        // PATHが空だと ls や cd すら困難になるため、現在のPATHを継承
        .env("PATH", env::var("PATH").unwrap_or_default())
        // TERMがないと画面が崩れたり、色が使えなかったりします
        .env("TERM", env::var("TERM").unwrap_or_default())
        // 一部のプログラムが参照するため USER も入れておくと安全です
        .env("USER", env::var("USER").unwrap_or_default())
        .current_dir(&run_dir);

    child
        .spawn()
        .expect("Failed to start zsh session")
        .wait()
        .expect("Failed to wait for zsh session");
}
