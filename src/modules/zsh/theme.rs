mod color_named_color;
pub mod color_scheme;
pub mod config_ui;
pub mod gradient;
pub mod manager;
pub mod named_color_serde; // 既存のファイルをそのまま使用
pub mod named_color_serde_option; // 新しく追加
pub mod prompt_theme;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;

use crate::zsh::theme::prompt_theme::{PromptContent, PromptContents};

pub async fn main() {
    let mut current_theme = manager::load_theme();
    loop {
        println!("\n--- Zsh Infinite Theme Configuration ---");

        // メインメニューのオプションを動的に生成
        let mut options = vec![
            "Add new prompt line".to_string(),
            "Remove last prompt line".to_string(),
        ];
        for (i, _) in current_theme.prompt_contents_list.iter().enumerate() {
            options.push(format!("Configure Prompt Line {}", i));
        }
        options.push("Save and Exit".to_string());

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Main Menu")
            .items(&options)
            .interact()
            .unwrap();

        match selection {
            0 => {
                // Add new prompt line
                current_theme
                    .prompt_contents_list
                    .push(PromptContents::default());
                println!("New prompt line added.");
            }
            1 => {
                // Remove last prompt line
                if current_theme.prompt_contents_list.pop().is_some() {
                    println!("Last prompt line removed.");
                } else {
                    println!("No prompt lines to remove.");
                }
            }
            s if s >= 2 && s < options.len() - 1 => {
                // Configure Prompt Line
                let line_index = s - 2;
                if let Some(prompt_contents) =
                    current_theme.prompt_contents_list.get_mut(line_index)
                {
                    configure_prompt_line(prompt_contents).await;
                } else {
                    eprintln!("Invalid prompt line index selected.");
                }
            }
            s if s == options.len() - 1 => {
                // Save and Exit
                let _ = manager::save_theme(&current_theme);
                break;
            }
            _ => unreachable!(),
        }
    }
}

async fn configure_prompt_line(prompt_contents: &mut PromptContents) {
    loop {
        println!("\n--- Configure Prompt Line ---");
        let options = [
            "Configure Colors",
            "Configure Connection",
            "Configure Separators",
            "Configure Left Prompt Content",  // 新しいオプション
            "Configure Right Prompt Content", // 新しいオプション
            "Back to Main Menu",
        ];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Prompt Line Configuration")
            .items(options)
            .interact()
            .unwrap();

        match selection {
            0 => config_ui::configure_colors(prompt_contents),
            1 => config_ui::configure_connection(prompt_contents),
            2 => config_ui::configure_separation(prompt_contents),
            3 => configure_prompt_content_list(&mut prompt_contents.left, "Left"),
            4 => configure_prompt_content_list(&mut prompt_contents.right, "Right"),
            5 => break,
            _ => unreachable!(),
        }
    }
}

fn configure_prompt_content_list(contents: &mut [PromptContent], side: &str) {
    loop {
        println!("\n--- Configure {} Prompt Contents ---", side);

        // ラベルの動的生成
        let mut options: Vec<String> = contents
            .iter()
            .enumerate()
            .map(|(i, pc)| {
                let label = if let Some(cmd) = &pc.cmd {
                    format!("External: {}", cmd)
                } else if let Some(build_in) = &pc.build_in {
                    // Commands enumのバリアント名を表示（Debugトレイトが必要）
                    format!("Built-in: {:?}", build_in)
                } else if let Some(literal) = &pc.literal {
                    format!("Literal: {}", literal)
                } else {
                    "Empty Content".to_string()
                };
                format!("{}: {}", i, label)
            })
            .collect();

        options.push("Back to Prompt Line Menu".to_string());

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("Select {} content to configure", side))
            .items(&options)
            .interact()
            .unwrap();

        if selection == options.len() - 1 {
            break; // Back to Prompt Line Menu
        } else if let Some(prompt_content) = contents.get_mut(selection) {
            // ここでコンテンツごとの詳細編集（cmdの書き換えやbuild_inの切り替えなど）
            // を行うサブメニューを呼び出すことも検討してください。
            config_ui::configure_prompt_content_colors(prompt_content);
        } else {
            eprintln!("Invalid selection.");
        }
    }
}
