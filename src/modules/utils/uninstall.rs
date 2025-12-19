use super::paths;
use std::{env, fs, path::PathBuf};

pub fn uninstall() {
    let install_paths = match paths::get_install_paths() {
        Ok(paths) => paths,
        Err(e) => {
            eprintln!("Error determining installation paths: {}", e);
            return;
        }
    };

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

    // 4. Revert user's ~/.zshrc
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
            Ok(zshrc_content) => {
                let source_line = format!(
                    "source \"{}\"",
                    install_paths.zshrc_snippet_path.to_string_lossy()
                );
                let installer_comment_start = "# Added by zsh-infinite installer";

                let original_len = zshrc_content.len();

                // Remove the source line and the comment
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
                    match fs::write(&user_zshrc_path, updated_content) {
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
            Err(e) => eprintln!("Error reading ~/.zshrc: {}", e),
        }
    } else {
        println!(
            "User's ~/.zshrc not found at {:?}. No changes to revert.",
            user_zshrc_path
        );
    }

    // 5. Clean up empty directories
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

    // Try to remove the config directory if it's empty
    let config_dir = install_paths
        .theme_file_path
        .parent()
        .expect("Theme file path should have a parent directory");
    if config_dir.exists() {
        match fs::remove_dir(config_dir) {
            Ok(_) => println!("Removed empty directory: {:?}", config_dir),
            Err(_) => println!(
                "Directory {:?} is not empty or could not be removed.",
                config_dir
            ),
        }
    }

    println!("\nUninstallation complete!");
}
