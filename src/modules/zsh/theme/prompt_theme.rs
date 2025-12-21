use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::process::Command;

use super::color_scheme::PromptColorScheme;
use crate::zsh::prompt::{PromptConnection, PromptSeparation};

#[derive(Clone, Debug, Serialize, Deserialize, Default, Copy)]
pub enum AccentWhich {
    #[default]
    ForeGround,
    BackGround,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PromptTheme {
    pub prompt_contents_list: Vec<PromptContents>,
    #[serde(default)]
    pub transient_color: PromptColorScheme,
}

impl Default for PromptTheme {
    fn default() -> Self {
        Self {
            prompt_contents_list: vec![PromptContents::default()],
            transient_color: PromptColorScheme::default(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PromptSegmentSeparators {
    pub start_separator: PromptSeparation,
    pub mid_separator: PromptSeparation,
    pub end_separator: PromptSeparation,
}

impl Default for PromptSegmentSeparators {
    fn default() -> Self {
        Self {
            start_separator: PromptSeparation::Sharp,
            mid_separator: PromptSeparation::Sharp,
            end_separator: PromptSeparation::Sharp,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PromptContents {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub left: Vec<PromptContent>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub right: Vec<PromptContent>,
    #[serde(default)]
    pub color: super::color_scheme::PromptColorScheme,
    pub connection: PromptConnection,
    pub left_segment_separators: PromptSegmentSeparators,
    pub right_segment_separators: PromptSegmentSeparators,
    pub accent_which: AccentWhich,
}

impl Default for PromptContents {
    fn default() -> Self {
        Self {
            left: vec![
                PromptContent::new(
                    "zsh".to_string(),
                    vec!["-c".to_string(), "whoami".to_string()],
                    vec![],
                ),
                PromptContent::new(
                    "zsh".to_string(),
                    vec!["-c".to_string(), "hostname".to_string()],
                    vec![],
                ),
            ],
            right: vec![
                PromptContent::new(
                    "zsh".to_string(),
                    vec!["-c".to_string(), "echo ${PWD/#$HOME/\\~}".to_string()],
                    vec![],
                ),
                // 終了コードの例（呼び出し側で調整される前提）
                PromptContent::new(
                    "zsh".to_string(),
                    vec!["-c".to_string(), "echo $LAST_STATUS".to_string()],
                    vec![],
                ),
            ],
            color: super::color_scheme::PromptColorScheme::default(),
            connection: PromptConnection::default(),
            left_segment_separators: PromptSegmentSeparators::default(),
            right_segment_separators: PromptSegmentSeparators::default(),
            accent_which: AccentWhich::default(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PromptContent {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub envs: Vec<HashMap<String, String>>,
    pub cmd: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,
}

impl PromptContent {
    pub fn new(cmd: String, args: Vec<String>, envs: Vec<HashMap<String, String>>) -> Self {
        Self { envs, cmd, args }
    }

    pub async fn content(&self) -> Option<String> {
        if self.cmd.is_empty() {
            return None;
        }

        // Command::new はデフォルトで現在のプロセスの環境変数を継承します
        let mut command = Command::new(&self.cmd);

        // 引数の設定
        command.args(&self.args);
        if let Ok(current_dir) = std::env::current_dir() {
            command.current_dir(current_dir);
        }
        // 個別の環境変数を適用（継承したものに追加・上書き）
        for env_map in &self.envs {
            for (key, value) in env_map {
                command.env(key, value);
            }
        }

        let output = command.output().await.ok()?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if stdout.is_empty() {
                None
            } else {
                Some(stdout)
            }
        } else {
            // コマンド実行に失敗した場合の処理。
            // 0以外のステータス時に空文字以外の標準出力がある場合はそれを返す仕様にすることも可能です。
            None
        }
    }
}
