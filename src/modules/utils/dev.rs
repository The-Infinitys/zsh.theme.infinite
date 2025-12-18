use std::{env, fs, process::Command};

pub fn dev() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let run_dir = current_dir.join("run");
    let zshrc_path = run_dir.join(".zshrc");
    let infinite_theme_path = run_dir.join("infinite.zsh-theme");

    // Create run directory if it doesn't exist
    if !run_dir.exists() {
        fs::create_dir(&run_dir).expect("Failed to create run directory");
        println!("Created directory: {:?}", run_dir);
    }
    let zshrc_content = include_str!("../../assets/scripts/dev.zshrc");
    let dev_zsh_theme = include_str!("../../assets/scripts/dev.zsh-theme");
    // プレースホルダを実行時の値で置換する
    let zshrc_content = zshrc_content.replace("{{RUN_DIR}}", &run_dir.to_string_lossy());
    fs::write(&zshrc_path, zshrc_content).expect("Failed to write .zshrc");
    println!("Created .zshrc at: {:?}", zshrc_path);

    // Copy infinite.zsh-theme
    fs::write(&infinite_theme_path, dev_zsh_theme).expect("Failed to copy infinite.zsh-theme");
    println!("Copied theme to: {:?}", infinite_theme_path);

    // Start a new zsh session
    println!("Starting new zsh session with run directory as home...");
    Command::new("zsh")
        // HOMEを書き換えるのではなく、Zshの設定参照先(ZDOTDIR)をrun_dirに固定する
        .env("ZDOTDIR", &run_dir)
        // 必要に応じてHOMEも維持または変更
        .env("HOME", &run_dir)
        .spawn()
        .expect("Failed to start zsh session")
        .wait()
        .expect("Failed to wait for zsh session");
}
