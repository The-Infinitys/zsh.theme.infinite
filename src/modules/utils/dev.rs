use std::{env, fs, process::Command};

pub fn dev() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let run_dir = current_dir.join("run");

    // --- 1. ディレクトリ作成 ---
    if run_dir.exists() {
        fs::remove_dir_all(&run_dir).expect("Failed to clear run directory")
    }
    fs::create_dir(&run_dir).expect("Failed to create run directory");

    // --- 2. 自分自身のバイナリをコピー ---
    let exe_path = env::current_exe().expect("Failed to get current executable path");
    let exe_name = exe_path.file_name().expect("Failed to get file name");
    let target_path = run_dir.join(exe_name);
    fs::copy(&exe_path, &target_path).expect("Failed to copy self");

    // --- 3. シェルの判定 ---
    // あなたの ShellType::from_env() を使うか、ここで簡易判定
    let shell_full_path = env::var("SHELL").unwrap_or_else(|_| "/bin/zsh".to_string());
    let is_bash = shell_full_path.contains("bash");
    let shell_name = if is_bash { "bash" } else { "zsh" };

    // --- 4. 設定ファイルの作成 ---
    let (rc_filename, theme_filename, rc_template, theme_template) = if is_bash {
        (
            ".bashrc",
            ".bash-theme",
            include_str!("../../assets/scripts/dev.bashrc"),
            include_str!("../../assets/scripts/dev.bash-theme"),
        )
    } else {
        (
            ".zshrc",
            ".zsh-theme",
            include_str!("../../assets/scripts/dev.zshrc"),
            include_str!("../../assets/scripts/dev.zsh-theme"),
        )
    };

    let rc_path = run_dir.join(rc_filename);
    let theme_path = run_dir.join(theme_filename);

    let rc_content = rc_template.replace("{{RUN_DIR}}", &run_dir.to_string_lossy());
    let theme_content = theme_template.replace("{{RUN_DIR}}", &run_dir.to_string_lossy());

    fs::write(&rc_path, rc_content).expect("Failed to write RC file");
    fs::write(&theme_path, theme_content).expect("Failed to write theme file");

    // --- 5. プログラムの起動 ---
    println!("Starting clean {} session...", shell_name);

    let mut child = Command::new(shell_name);

    child
        .env_clear()
        .env("HOME", &run_dir)
        .env("PATH", env::var("PATH").unwrap_or_default())
        .env("TERM", env::var("TERM").unwrap_or_default())
        .env("USER", env::var("USER").unwrap_or_default())
        // あなたのプログラムが SHELL 変数を見て挙動を変えるので、これを明示的にセット
        .env("SHELL", &shell_full_path)
        .current_dir(&run_dir);

    // Bashの場合、標準では ~/.bashrc を見に行くため、--rcfile で作成したファイルを指定する
    if is_bash {
        child.arg("--rcfile").arg(&rc_path);
    } else {
        // Zshの場合は ZDOTDIR で設定ファイルの場所を制御
        child.env("ZDOTDIR", &run_dir);
    }

    child
        .spawn()
        .expect("Failed to start shell session")
        .wait()
        .expect("Failed to wait for shell session");
}
