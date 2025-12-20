use super::paths;
use std::{env, fs, path::PathBuf};

pub fn install() {
    let install_paths = match paths::get_install_paths() {
        Ok(paths) => paths,
        Err(e) => {
            eprintln!("Error determining installation paths: {}", e);
            return;
        }
    };

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

    // 3. Generate infinite.zsh-theme
    let theme_template = include_str!("../../assets/scripts/infinite.zsh-theme").to_string();
    let theme_content = theme_template.replace(
        "{{RUN_DIR}}",
        &target_exe_path.to_string_lossy(),
    );

    // Create theme directory if it doesn't exist
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
        // If .zshrc doesn't exist, we can't modify it, so just exit
        return;
    }

    match fs::read_to_string(&user_zshrc_path) {
        Ok(mut zshrc_content) => {
            if install_paths.is_oh_my_zsh_install {
                // Oh My Zsh installation
                let zsh_var = format!("ZSH=\"{}\"", paths::get_oh_my_zsh_root().unwrap().to_string_lossy());
                let zsh_custom_var = format!("ZSH_CUSTOM=\"{}\"", paths::get_oh_my_zsh_custom_theme_dir().unwrap().parent().unwrap().to_string_lossy());
                let source_oh_my_zsh = "source $ZSH/oh-my-zsh.sh";
                let theme_setting = format!("ZSH_THEME=\"{}\"", install_paths.theme_file_path.file_stem().unwrap().to_string_lossy());

                
                let mut _modified = false;

                // Add ZSH variable if not present
                if !zshrc_content.contains(&zsh_var) {
                    zshrc_content.push_str(&format!("\n{}", zsh_var));
                    _modified = true;
                }

                // Add ZSH_CUSTOM variable if not present
                if !zshrc_content.contains(&zsh_custom_var) {
                    zshrc_content.push_str(&format!("\n{}", zsh_custom_var));
                    _modified = true;
                }

                // Add source oh-my-zsh.sh if not present
                if !zshrc_content.contains(source_oh_my_zsh) {
                    zshrc_content.push_str(&format!("\n{}", source_oh_my_zsh));
                    _modified = true;
                }
                
                // Set ZSH_THEME
                // If ZSH_THEME is already set, replace it. Otherwise, add it.
                let theme_regex = regex::Regex::new(r"(?m)^ZSH_THEME=.*$").unwrap();
                if theme_regex.is_match(&zshrc_content) {
                    zshrc_content = theme_regex.replace(&zshrc_content, theme_setting.as_str()).to_string();
                    _modified = true;
                } else {
                    zshrc_content.push_str(&format!("\n{}", theme_setting));
                    _modified = true;
                }


                if _modified {
                    match fs::write(&user_zshrc_path, zshrc_content) {
                        Ok(_) => println!("~/.zshrc updated for Oh My Zsh theme."),
                        Err(e) => eprintln!("Error writing to ~/.zshrc: {}", e),
                    }
                } else {
                    println!("~/.zshrc already configured for Oh My Zsh theme. Skipping modification.");
                }

            } else {
                // Standalone installation
                let source_line = format!(
                    "source \"{}\"",
                    install_paths.zshrc_snippet_path.to_string_lossy()
                );
                let installer_comment_start = "# Added by zsh-infinite installer";

                if !zshrc_content.contains(&source_line) {
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

                    zshrc_content
                        .push_str(&format!("\n{}\n{}\n", installer_comment_start, source_line));
                    match fs::write(&user_zshrc_path, zshrc_content) {
                        Ok(_) => println!("'{}' added to ~/.zshrc.", source_line),
                        Err(e) => eprintln!("Error writing to ~/.zshrc: {}", e),
                    }
                } else {
                    println!(
                        "'{}' already present in ~/.zshrc. Skipping modification.",
                        source_line
                    );
                }
            }
        }
        Err(e) => eprintln!("Error reading ~/.zshrc: {}", e),
    }

    println!(
        "\nInstallation complete! Please restart your Zsh session or run 'source ~/.zshrc' to apply the changes."
    );
}
