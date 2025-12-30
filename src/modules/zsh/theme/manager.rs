use directories::ProjectDirs;
use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};

use crate::zsh::theme::prompt_theme::PromptTheme;

const QUALIFIER: &str = "org";
const ORGANIZATION: &str = "infinite";
const APPLICATION: &str = "zsh-infinite";
const THEME_FILE_NAME: &str = "theme.yaml";

fn get_theme_file_path() -> Option<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
        let config_dir = proj_dirs.config_dir();
        let theme_file_path = config_dir.join(THEME_FILE_NAME);
        Some(theme_file_path)
    } else {
        None
    }
}

pub fn load_theme() -> PromptTheme {
    if let Some(theme_file_path) = get_theme_file_path() {
        if theme_file_path.exists() {
            match fs::read_to_string(&theme_file_path) {
                Ok(content) => match serde_yaml::from_str(&content) {
                    Ok(theme) => {
                        eprintln!("Theme loaded successfully from: {:?}", theme_file_path);
                        theme
                    }
                    Err(e) => {
                        eprintln!(
                            "Failed to deserialize theme from {:?}: {}",
                            theme_file_path, e
                        );
                        PromptTheme::default()
                    }
                },
                Err(e) => {
                    eprintln!("Failed to read theme file {:?}: {}", theme_file_path, e);
                    PromptTheme::default()
                }
            }
        } else {
            eprintln!("Theme file not found at {:?}", theme_file_path);
            let default_theme = PromptTheme::default();
            // Attempt to save the default theme if the file doesn't exist
            if let Err(e) = save_theme(&default_theme) {
                eprintln!("Error saving default theme to {:?}: {}", theme_file_path, e);
            } else {
                eprintln!("Default theme saved to {:?}", theme_file_path);
            }
            default_theme
        }
    } else {
        eprintln!("Could not determine project directories for theme file.");
        PromptTheme::default()
    }
}

pub fn save_theme(theme: &PromptTheme) -> io::Result<()> {
    if let Some(theme_file_path) = get_theme_file_path() {
        let config_dir = theme_file_path
            .parent()
            .expect("Theme file path should have a parent directory");
        fs::create_dir_all(config_dir)?;

        let content = serde_yaml::to_string(theme).map_err(|e| io::Error::other(e.to_string()))?;

        let mut file = fs::File::create(&theme_file_path)?;
        file.write_all(content.as_bytes())?;
        eprintln!("Theme saved successfully to: {:?}", theme_file_path);
        Ok(())
    } else {
        Err(io::Error::other(
            "Could not determine project directories to save theme file.",
        ))
    }
}
