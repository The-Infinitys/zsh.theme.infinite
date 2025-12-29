use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::HashMap};
use tokio::process::Command;
use zsh_seq::{NamedColor, ZshSequence};

use super::color_scheme::PromptColorScheme;
// 変更
use crate::zsh::{
    prompt::{PromptConnection, PromptSeparation},
    theme::color_named_color::ToNamedColor,
};

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
    pub edge_cap: bool,
    pub bold_separation: bool,
}

impl Default for PromptSegmentSeparators {
    fn default() -> Self {
        Self {
            start_separator: PromptSeparation::Sharp,
            mid_separator: PromptSeparation::Sharp,
            end_separator: PromptSeparation::Sharp,
            edge_cap: true,
            bold_separation: true,
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
                PromptContent::shell(
                    "zsh".to_string(),
                    vec!["-c".to_string(), "whoami".to_string()],
                    vec![],
                    None,
                    None,
                ),
                PromptContent::shell(
                    "zsh".to_string(),
                    vec!["-c".to_string(), "hostname".to_string()],
                    vec![],
                    None,
                    None,
                ),
            ],
            right: vec![
                PromptContent::shell(
                    "zsh".to_string(),
                    vec!["-c".to_string(), "echo ${PWD/#$HOME/\\~}".to_string()],
                    vec![],
                    None,
                    None,
                ),
                // 終了コードの例（呼び出し側で調整される前提）
                PromptContent::shell(
                    "zsh".to_string(),
                    vec!["-c".to_string(), "echo $LAST_STATUS".to_string()],
                    vec![],
                    None,
                    None,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub literal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_in: Option<zsh_prompts::Commands>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub envs: Vec<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmd: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,
    #[serde(
        with = "super::named_color_serde_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fg_color: Option<NamedColor>,
    #[serde(
        with = "super::named_color_serde_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bg_color: Option<NamedColor>,
}

impl PromptContent {
    pub fn shell(
        cmd: String,
        args: Vec<String>,
        envs: Vec<HashMap<String, String>>,
        fg_color: Option<NamedColor>,
        bg_color: Option<NamedColor>,
    ) -> Self {
        Self {
            literal: None,
            build_in: None,
            envs,
            cmd: Some(cmd),
            args,
            fg_color,
            bg_color,
        }
    }
    pub fn build_in(build_in: zsh_prompts::Commands) -> Self {
        Self {
            literal: None,
            build_in: Some(build_in),
            envs: vec![],
            cmd: None,
            args: vec![],
            fg_color: None,
            bg_color: None,
        }
    }
    pub fn literal(
        literal: String,
        fg_color: Option<NamedColor>,
        bg_color: Option<NamedColor>,
    ) -> Self {
        Self {
            literal: Some(literal),
            build_in: None,
            envs: vec![],
            cmd: None,
            args: vec![],
            fg_color,
            bg_color,
        }
    }
    pub async fn content(&self) -> Vec<ZshSequence> {
        // 1. Literal の処理
        if let Some(literal) = &self.literal {
            return vec![ZshSequence::Literal(literal.clone())];
        }

        // 2. Build-in コマンドの処理
        // 2. Build-in コマンドの処理
        if let Some(build_in) = &self.build_in {
            let segments = build_in.exec();
            let mut result = Vec::new();
            let len = segments.len();

            for (i, segment) in segments.into_iter().enumerate() {
                // 1. 色の開始 (Foreground)
                if let Some(fg) = segment.color {
                    result.push(ZshSequence::ForegroundColor(fg.to_named_color()));
                }

                // 2. コンテンツ本体
                result.push(ZshSequence::Literal(segment.content));

                // 3. 色の終了
                if segment.color.is_some() {
                    result.push(ZshSequence::ForegroundColorEnd);
                }

                // 4. セグメント間のスペース挿入
                // 最後のセグメント以外、かつ現在のセグメントが空でない場合にスペースを入れる
                if i < len - 1 {
                    result.push(ZshSequence::Literal(" ".to_string()));
                }
            }
            return result;
        }

        // 3. 外部コマンド (Shell) の処理
        if let Some(cmd) = &self.cmd {
            let mut command = Command::new(cmd);

            let expanded_args: Vec<String> = self
                .args
                .iter()
                .map(|arg| {
                    shellexpand::env(arg)
                        .unwrap_or(Cow::Borrowed(arg))
                        .to_string()
                })
                .collect();

            command.args(&expanded_args);

            if let Ok(current_dir) = std::env::current_dir() {
                command.current_dir(current_dir);
            }

            for env_map in &self.envs {
                for (key, value) in env_map {
                    command.env(key, value);
                }
            }

            if let Ok(output) = command.output().await {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !stdout.is_empty() {
                        let mut seqs = Vec::new();

                        // 背景色
                        if let Some(bg) = self.bg_color {
                            seqs.push(ZshSequence::BackgroundColor(bg));
                        }
                        // 前景色
                        if let Some(fg) = self.fg_color {
                            seqs.push(ZshSequence::ForegroundColor(fg));
                        }

                        seqs.push(ZshSequence::Literal(stdout));

                        // 終了タグ (順番に注意)
                        if self.fg_color.is_some() {
                            seqs.push(ZshSequence::ForegroundColorEnd);
                        }
                        if self.bg_color.is_some() {
                            seqs.push(ZshSequence::BackgroundColorEnd);
                        }

                        return seqs;
                    }
                }
            }
        }

        // 失敗、または該当なしの場合は空配列を返す
        Vec::new()
    }
}
