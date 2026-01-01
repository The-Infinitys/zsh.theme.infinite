use super::paths;
use regex::Regex;
use std::{
    env::{self, home_dir},
    fs,
    path::PathBuf,
};

pub fn uninstall() {
    let install_paths = match paths::get_install_paths() {
        Ok(paths) => paths,
        Err(e) => {
            eprintln!("Error determining installation paths: {}", e);
            return;
        }
    };
    // --- 追加: ライブラリの削除処理 ---
    let lib_dir = home_dir().unwrap().join(".local/lib");
    let lib_file_name = if cfg!(target_os = "macos") {
        "libzsh_infinite.dylib"
    } else {
        "libzsh_infinite.so"
    };
    let lib_path = lib_dir.join(lib_file_name);

    if lib_path.exists() {
        // 現在のプロセスがライブラリをロードしているかチェック
        // (zsh-infinite バイナリ自身が zmodload されているわけではないため、通常は削除可能)
        match fs::remove_file(&lib_path) {
            Ok(_) => println!("Removed shared library: {:?}", lib_path),
            Err(e) => {
                // 使用中の場合は unlink に失敗することがある
                eprintln!("Could not remove library file (it might be in use): {}", e);
            }
        }
    }
    // 1. Remove the executable
    let current_exe_name = match env::current_exe() {
        Ok(path) => path.file_name().map(|s| s.to_os_string()),
        Err(_) => None,
    };

    if let Some(exe_name) = current_exe_name {
        let target_exe_path = install_paths.bin_dir.join(exe_name);
        if target_exe_path.exists() {
            match fs::remove_file(&target_exe_path) {
                Ok(_) => println!("Removed executable: {:?}", target_exe_path),
                Err(e) => eprintln!("Error removing executable {:?}: {}", target_exe_path, e),
            }
        } else {
            println!("Executable not found at {:?}", target_exe_path);
        }
    } else {
        eprintln!("Could not determine current executable name to remove.");
    }

    // 2. Remove theme file
    if install_paths.theme_file_path.exists() {
        match fs::remove_file(&install_paths.theme_file_path) {
            Ok(_) => println!("Removed theme file: {:?}", install_paths.theme_file_path),
            Err(e) => eprintln!(
                "Error removing theme file {:?}: {}",
                install_paths.theme_file_path, e
            ),
        }
    } else {
        println!(
            "Theme file not found at {:?}",
            install_paths.theme_file_path
        );
    }

    // 3. Modify user's ~/.zshrc
    let home_dir = match env::var("HOME") {
        Ok(dir) => PathBuf::from(dir),
        Err(_) => {
            eprintln!("Error: HOME environment variable not set.");
            return;
        }
    };
    let user_zshrc_path = home_dir.join(".zshrc");

    if user_zshrc_path.exists() {
        match fs::read_to_string(&user_zshrc_path) {
            Ok(mut zshrc_content) => {
                if install_paths.is_oh_my_zsh_install {
                    // Oh My Zsh uninstallation
                    let theme_name = install_paths
                        .theme_file_path
                        .file_stem()
                        .unwrap()
                        .to_string_lossy();
                    let zsh_root_path_str = paths::get_oh_my_zsh_root()
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or_default();
                    let zsh_custom_path_str = paths::get_oh_my_zsh_custom_theme_dir()
                        .map(|p| p.parent().unwrap().to_string_lossy().to_string())
                        .unwrap_or_default();

                    let zsh_var_line = format!("ZSH=\"{}\"", zsh_root_path_str);
                    let zsh_custom_var_line = format!("ZSH_CUSTOM=\"{}\"", zsh_custom_path_str);
                    let source_oh_my_zsh_line = "source $ZSH/oh-my-zsh.sh";

                    let original_len = zshrc_content.len();

                    // Remove ZSH_THEME setting
                    let theme_regex = Regex::new(&format!(
                        "(?m)^ZSH_THEME=\"{}\"$",
                        regex::escape(&theme_name)
                    ))
                    .unwrap();
                    zshrc_content = theme_regex.replace_all(&zshrc_content, "").to_string();

                    // Remove ZSH and ZSH_CUSTOM variable settings if they match exactly what we added
                    let zsh_var_regex =
                        Regex::new(&format!("(?m)^{}$", regex::escape(&zsh_var_line))).unwrap();
                    zshrc_content = zsh_var_regex.replace_all(&zshrc_content, "").to_string();

                    let zsh_custom_var_regex =
                        Regex::new(&format!("(?m)^{}$", regex::escape(&zsh_custom_var_line)))
                            .unwrap();
                    zshrc_content = zsh_custom_var_regex
                        .replace_all(&zshrc_content, "")
                        .to_string();

                    // Remove source oh-my-zsh.sh
                    let source_oh_my_zsh_regex =
                        Regex::new(&format!("(?m)^{}$", regex::escape(source_oh_my_zsh_line)))
                            .unwrap();
                    zshrc_content = source_oh_my_zsh_regex
                        .replace_all(&zshrc_content, "")
                        .to_string();

                    // Remove any consecutive blank lines
                    let blank_line_regex = Regex::new(r"\n\n+").unwrap();
                    zshrc_content = blank_line_regex
                        .replace_all(&zshrc_content, "\n")
                        .to_string();

                    // Trim leading/trailing newlines
                    zshrc_content = zshrc_content.trim_start().trim_end().to_string();

                    if original_len != zshrc_content.len() {
                        match fs::write(&user_zshrc_path, zshrc_content.as_bytes()) {
                            Ok(_) => {
                                println!("~/.zshrc updated to remove Oh My Zsh theme settings.")
                            }
                            Err(e) => eprintln!("Error writing to ~/.zshrc: {}", e),
                        }
                    } else {
                        println!(
                            "Oh My Zsh theme settings not found in ~/.zshrc. Skipping modification."
                        );
                    }
                } else {
                    // Standalone uninstallation
                    // 3. Remove zshrc snippet file
                    if install_paths.zshrc_snippet_path.exists() {
                        match fs::remove_file(&install_paths.zshrc_snippet_path) {
                            Ok(_) => println!(
                                "Removed zshrc snippet file: {:?}",
                                install_paths.zshrc_snippet_path
                            ),
                            Err(e) => eprintln!(
                                "Error removing zshrc snippet file {:?}: {}",
                                install_paths.zshrc_snippet_path, e
                            ),
                        }
                    } else {
                        println!(
                            "Zshrc snippet file not found at {:?}",
                            install_paths.zshrc_snippet_path
                        );
                    }

                    let source_line = format!(
                        "source \"{}\"",
                        install_paths.zshrc_snippet_path.to_string_lossy()
                    );
                    let installer_comment_start = "# Added by zsh-infinite installer";

                    let original_len = zshrc_content.len();

                    let updated_content = zshrc_content
                        .replace(
                            &format!("\n{}\n{}\n", installer_comment_start, source_line),
                            "\n",
                        )
                        .replace(&format!("\n{}\n", source_line), "\n") // Fallback in case comment was not there
                        .replace(
                            &format!("{}\n{}\n", installer_comment_start, source_line),
                            "",
                        ) // For start of file without preceding newline
                        .replace(&source_line, ""); // Final fallback for line itself

                    if updated_content.len() != original_len {
                        match fs::write(&user_zshrc_path, updated_content.as_bytes()) {
                            Ok(_) => println!("Removed '{}' from ~/.zshrc.", source_line),
                            Err(e) => eprintln!("Error writing to ~/.zshrc: {}", e),
                        }
                    } else {
                        println!(
                            "'{}' not found or already removed from ~/.zshrc. Skipping modification.",
                            source_line
                        );
                    }
                }
            }
            Err(e) => eprintln!("Error reading ~/.zshrc: {}", e),
        }
    } else {
        println!(
            "User's ~/.zshrc not found at {:?}. No changes to revert.",
            user_zshrc_path
        );
    }

    // 4. Clean up empty directories
    // Try to remove the bin directory if it's empty
    if install_paths.bin_dir.exists() {
        match fs::remove_dir(&install_paths.bin_dir) {
            Ok(_) => println!("Removed empty directory: {:?}", install_paths.bin_dir),
            Err(_) => println!(
                "Directory {:?} is not empty or could not be removed.",
                install_paths.bin_dir
            ),
        }
    }

    // Try to remove the config directory if it's empty (only for standalone)
    if !install_paths.is_oh_my_zsh_install {
        let config_dir_parent = install_paths
            .zshrc_snippet_path
            .parent()
            .expect("Zshrc snippet path should have a parent directory");
        if config_dir_parent.exists() {
            // Check if it's empty before trying to remove
            if fs::read_dir(config_dir_parent).is_ok_and(|mut dir| dir.next().is_none()) {
                match fs::remove_dir(config_dir_parent) {
                    Ok(_) => println!("Removed empty directory: {:?}", config_dir_parent),
                    Err(_) => println!(
                        "Directory {:?} is not empty or could not be removed.",
                        config_dir_parent
                    ),
                }
            } else {
                println!(
                    "Directory {:?} is not empty. Skipping removal.",
                    config_dir_parent
                );
            }
        }
    }

    println!("\nUninstallation complete!");
}
