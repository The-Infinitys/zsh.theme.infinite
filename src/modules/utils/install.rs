use super::paths;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::{
    env::{self, home_dir},
    fs,
    path::PathBuf,
};

const LIB_DATA: &[u8] = include_bytes!(env!("ZSH_LIB_PATH"));

pub fn install() {
    let install_paths = match paths::get_install_paths() {
        Ok(paths) => paths,
        Err(e) => {
            eprintln!("Error determining installation paths: {}", e);
            return;
        }
    };
    let lib_dir = home_dir().unwrap().join(".local/lib");
    if let Err(e) = fs::create_dir_all(&lib_dir) {
        eprintln!("Error creating library directory {:?}: {}", lib_dir, e);
        return;
    }

    // OSに応じた拡張子の決定
    let lib_file_name = if cfg!(target_os = "macos") {
        "libzsh_infinite.dylib"
    } else {
        "libzsh_infinite.so"
    };
    let lib_dest_path = lib_dir.join(lib_file_name);

    match fs::write(&lib_dest_path, LIB_DATA) {
        Ok(_) => {
            println!("Shared library installed to: {:?}", lib_dest_path);
            // 実行権限の付与（必要に応じて）
            #[cfg(unix)]
            let _ = fs::set_permissions(&lib_dest_path, fs::Permissions::from_mode(0o755));
        }
        Err(e) => {
            eprintln!(
                "Error installing shared library to {:?}: {}",
                lib_dest_path, e
            );
            return;
        }
    }
    // 1. Create necessary directories for bin
    if let Err(e) = fs::create_dir_all(&install_paths.bin_dir) {
        eprintln!(
            "Error creating binary directory {:?}: {}",
            install_paths.bin_dir, e
        );
        return;
    }

    // 2. Copy the current executable
    let current_exe_path = match env::current_exe() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Error getting current executable path: {}", e);
            return;
        }
    };
    let target_exe_name = current_exe_path
        .file_name()
        .expect("Failed to get file name");
    let target_exe_path = install_paths.bin_dir.join(target_exe_name);

    if current_exe_path == target_exe_path {
        println!(
            "Executable already exists at target path: {:?}",
            target_exe_path
        );
    } else {
        match fs::copy(&current_exe_path, &target_exe_path) {
            Ok(_) => println!("Executable copied to: {:?}", target_exe_path),
            Err(e) => {
                eprintln!(
                    "Error copying executable from {:?} to {:?}: {}",
                    current_exe_path, target_exe_path, e
                );
                return;
            }
        }
    }

    // 3. Generate infinite.zsh-theme
    let theme_template = include_str!("../../assets/scripts/infinite.zsh-theme").to_string();
    let theme_content = theme_template.replace(
        "{{RUN_DIR}}",
        &target_exe_path
            .parent()
            .expect("Binary path should have a parent directory")
            .to_string_lossy(),
    );

    if let Err(e) = fs::create_dir_all(
        install_paths
            .theme_file_path
            .parent()
            .expect("Theme file path should have a parent directory"),
    ) {
        eprintln!(
            "Error creating theme directory {:?}: {}",
            install_paths.theme_file_path.parent().unwrap(),
            e
        );
        return;
    }

    match fs::write(&install_paths.theme_file_path, theme_content) {
        Ok(_) => println!("Theme file created at: {:?}", install_paths.theme_file_path),
        Err(e) => {
            eprintln!(
                "Error writing theme file to {:?}: {}",
                install_paths.theme_file_path, e
            );
            return;
        }
    }

    // 4. Modify user's ~/.zshrc
    let home_dir = match env::var("HOME") {
        Ok(dir) => PathBuf::from(dir),
        Err(_) => {
            eprintln!("Error: HOME environment variable not set.");
            return;
        }
    };
    let user_zshrc_path = home_dir.join(".zshrc");

    if !user_zshrc_path.exists() {
        println!(
            "User's ~/.zshrc not found at {:?}. Please create it.",
            user_zshrc_path
        );
        return;
    }

    match fs::read_to_string(&user_zshrc_path) {
        Ok(zshrc_content) => {
            let mut _modified = false;
            let mut new_zshrc_content_lines: Vec<String> = Vec::new();

            if install_paths.is_oh_my_zsh_install {
                // Oh My Zsh installation
                let zsh_root = paths::get_oh_my_zsh_root().expect("Oh My Zsh root not found");
                let zsh_custom_parent = paths::get_oh_my_zsh_custom_theme_dir()
                    .expect("Oh My Zsh custom theme directory not found")
                    .parent()
                    .expect("ZSH_CUSTOM parent directory not found")
                    .to_path_buf();
                let theme_name = install_paths
                    .theme_file_path
                    .file_stem()
                    .expect("Theme file name not found")
                    .to_string_lossy();

                let zsh_var_line = format!("ZSH=\"{}\"", zsh_root.to_string_lossy());
                let zsh_custom_var_line =
                    format!("ZSH_CUSTOM=\"{}\"", zsh_custom_parent.to_string_lossy());
                let theme_setting_line = format!("export ZSH_THEME=\"{}\"", theme_name); // export を追加
                let source_oh_my_zsh_line_exact = "source $ZSH/oh-my-zsh.sh";

                let oh_my_zsh_block = format!(
                    "{}\n{}\n{}\n{}",
                    zsh_var_line,
                    zsh_custom_var_line,
                    theme_setting_line,
                    source_oh_my_zsh_line_exact
                );

                let mut oh_my_zsh_source_found_in_original = false;
                let mut omz_block_inserted = false;

                for line in zshrc_content.lines() {
                    let trimmed_line = line.trim();

                    // 既存の Oh My Zsh 関連の行をスキップ
                    if trimmed_line.starts_with("ZSH=")
                        || trimmed_line.starts_with("ZSH_CUSTOM=")
                        || trimmed_line.starts_with("ZSH_THEME=")
                        || trimmed_line.starts_with("export ZSH_THEME=")
                    {
                        if !omz_block_inserted {
                            // まだブロックが挿入されていなければ、既存の変更があるので_modifiedをセット
                            _modified = true;
                        }
                        continue; // これらの行は新しいブロックとして挿入するのでスキップ
                    }

                    // source $ZSH/oh-my-zsh.sh の行を特定し、その位置に新しいブロックを挿入
                    if trimmed_line.contains(source_oh_my_zsh_line_exact) && !omz_block_inserted {
                        oh_my_zsh_source_found_in_original = true;
                        new_zshrc_content_lines.push(oh_my_zsh_block.clone());
                        _modified = true;
                        omz_block_inserted = true; // ブロック挿入済みフラグ
                    } else {
                        new_zshrc_content_lines.push(line.to_string());
                    }
                }

                // もし source $ZSH/oh-my-zsh.sh が元のファイル中に見つからなかった場合、末尾に追加
                if !oh_my_zsh_source_found_in_original && !omz_block_inserted {
                    if !new_zshrc_content_lines.is_empty() {
                        new_zshrc_content_lines.push(String::new()); // 前の行との間に改行を追加
                    }
                    new_zshrc_content_lines.push(oh_my_zsh_block.clone());
                    _modified = true;
                }

                // 最後の改行を削除し、内容を結合
                let final_zshrc_content = new_zshrc_content_lines.join("\n");

                if _modified {
                    match fs::write(&user_zshrc_path, final_zshrc_content.as_bytes()) {
                        Ok(_) => println!("~/.zshrc updated for Oh My Zsh theme."),
                        Err(e) => eprintln!("Error writing to ~/.zshrc: {}", e),
                    }
                } else {
                    println!(
                        "~/.zshrc already configured for Oh My Zsh theme. Skipping modification."
                    );
                }
            } else {
                // Standalone installation
                let source_line = format!(
                    "source \"{}\"",
                    install_paths.zshrc_snippet_path.to_string_lossy()
                );
                let installer_comment_start = "# Added by zsh-infinite installer";

                let mut zshrc_lines: Vec<String> =
                    zshrc_content.lines().map(|s| s.to_string()).collect();
                let mut source_line_present = false;

                // 既存の source line をチェックし、削除
                let original_line_count = zshrc_lines.len();
                zshrc_lines.retain(|line| {
                    if line.contains(&source_line) || line.contains(installer_comment_start) {
                        source_line_present = true;
                        return false; // 既存の行とコメントを削除
                    }
                    true
                });

                if zshrc_lines.len() != original_line_count {
                    _modified = true;
                }

                if !source_line_present {
                    if !zshrc_lines.is_empty() {
                        zshrc_lines.push(String::new()); // 前の行との間に改行を追加
                    }
                    zshrc_lines.push(installer_comment_start.to_string());
                    zshrc_lines.push(source_line.to_string());
                    _modified = true;

                    // Create snippet file
                    let zshrc_snippet_content = format!(
                        r#"
# Added by zsh-infinite installer
if [ -f "{}" ]; then
    source "{}"
fi
"#,
                        install_paths.theme_file_path.to_string_lossy(),
                        install_paths.theme_file_path.to_string_lossy()
                    );
                    if let Err(e) = fs::write(
                        &install_paths.zshrc_snippet_path,
                        zshrc_snippet_content.as_bytes(),
                    ) {
                        eprintln!(
                            "Error writing zshrc snippet to {:?}: {}",
                            install_paths.zshrc_snippet_path, e
                        );
                        return;
                    } else {
                        println!(
                            "Zshrc snippet created at: {:?}",
                            install_paths.zshrc_snippet_path
                        );
                    }
                }

                if _modified {
                    match fs::write(&user_zshrc_path, zshrc_lines.join("\n").as_bytes()) {
                        Ok(_) => println!("~/.zshrc updated for standalone theme."),
                        Err(e) => eprintln!("Error writing to ~/.zshrc: {}", e),
                    }
                }
            }
        }
        Err(e) => eprintln!("Error reading ~/.zshrc: {}", e),
    }

    println!(
        "\nInstallation complete! Please restart your Zsh session or run 'source ~/.zshrc' to apply the changes."
    );
}
