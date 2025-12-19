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

    // 1. Create necessary directories
    if let Err(e) = fs::create_dir_all(&install_paths.bin_dir) {
        eprintln!(
            "Error creating binary directory {:?}: {}",
            install_paths.bin_dir, e
        );
        return;
    }
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

    // 2. Copy the current executable
    let current_exe_path = match env::current_exe() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Error getting current executable path: {}", e);
            return;
        }
    };
    let target_exe_path = install_paths.bin_dir.join(
        current_exe_path
            .file_name()
            .expect("Failed to get file name"),
    );

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
    let theme_content = include_str!("../../assets/scripts/infinite.zsh-theme").to_string(); // Placeholder for now

    let theme_content_with_run_dir = theme_content.replace(
        "{{RUN_DIR}}", // This will be replaced if we have a template for it
        &target_exe_path.to_string_lossy(), // Use the actual executable path
    );

    match fs::write(&install_paths.theme_file_path, theme_content_with_run_dir) {
        Ok(_) => println!("Theme file created at: {:?}", install_paths.theme_file_path),
        Err(e) => {
            eprintln!(
                "Error writing theme file to {:?}: {}",
                install_paths.theme_file_path, e
            );
            return;
        }
    }

    // 4. Generate zshrc snippet to source the theme
    // For now, it's just a direct source.
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

    match fs::write(
        &install_paths.zshrc_snippet_path,
        zshrc_snippet_content.as_bytes(),
    ) {
        Ok(_) => println!(
            "Zshrc snippet created at: {:?}",
            install_paths.zshrc_snippet_path
        ),
        Err(e) => {
            eprintln!(
                "Error writing zshrc snippet to {:?}: {}",
                install_paths.zshrc_snippet_path, e
            );
            return;
        }
    }

    // 5. Modify user's ~/.zshrc
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
            "User's ~/.zshrc not found at {:?}. Please create it and manually add 'source {}'",
            user_zshrc_path,
            install_paths.zshrc_snippet_path.to_string_lossy()
        );
        return;
    }

    match fs::read_to_string(&user_zshrc_path) {
        Ok(mut zshrc_content) => {
            let source_line = format!(
                "source \"{}\"",
                install_paths.zshrc_snippet_path.to_string_lossy()
            );
            let installer_comment_start = "# Added by zsh-infinite installer";

            // Check if the source line (or the entire snippet block) is already present
            if !zshrc_content.contains(&source_line) {
                // Prepend the comment and the source line
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
        Err(e) => eprintln!("Error reading ~/.zshrc: {}", e),
    }

    println!(
        "\nInstallation complete! Please restart your Zsh session or run 'source ~/.zshrc' to apply the changes."
    );
}
