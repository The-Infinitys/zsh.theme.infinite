use crate::zsh::{
    prompt::{Prompt, PromptCurveLine},
    theme_manager,
};
use crossterm::terminal;
use futures::future::join_all;
use unicode_width::UnicodeWidthStr;
use zsh_seq::ZshPromptBuilder;

pub async fn left() {
    let theme = theme_manager::load_theme();
    let curved_lines = PromptCurveLine::from(theme.connection);

    let h = &curved_lines.horizontal;

    let list = &theme.prompt_contents_list;

    // 1. リストが空の場合の早期リターン（またはデフォルト表示）
    if list.is_empty() {
        // コンテンツがない場合でも、最低限の枠を表示して入力待ちにする
        let start = ZshPromptBuilder::new()
            .color(theme.color.sc)
            .str(&curved_lines.top_left)
            .str(h)
            .str(h)
            .str(&curved_lines.top_right)
            .end_color();
        println!("{}", start.build());

        let end = ZshPromptBuilder::new()
            .color(theme.color.sc)
            .str(&curved_lines.bottom_left)
            .str(h)
            .str(" ")
            .end_color();
        print!("{}", end.build());
        return;
    }

    // 2. リストがある場合のメインループ
    for (i, prompt_contents) in list.iter().enumerate() {
        let mut prompt = Prompt::default();

        // (非同期取得部分は変更なし)
        let left_futures: Vec<_> = prompt_contents
            .left
            .iter()
            .map(|c| async move { c.content().await })
            .collect();
        let right_futures: Vec<_> = prompt_contents
            .right
            .iter()
            .map(|c| async move { c.content().await })
            .collect();
        let (left_results, right_results) =
            tokio::join!(join_all(left_futures), join_all(right_futures));

        for content in left_results.into_iter().flatten() {
            prompt.add_left(&content);
        }
        for content in right_results.into_iter().flatten() {
            prompt.add_right(&content);
        }

        let left_content = prompt.render_left(&theme);
        let right_content = prompt.render_right(&theme);

        let terminal_width = terminal::size().map(|(w, _)| w).unwrap_or(80) as usize;
        let left_width = UnicodeWidthStr::width(left_content.text().as_str());
        let right_width = UnicodeWidthStr::width(right_content.text().as_str());

        let side_decor_width = 4;
        let connection_len =
            terminal_width.saturating_sub(left_width + right_width + side_decor_width);
        let connection = theme.connection.to_string().repeat(connection_len);

        let mut row_builder = ZshPromptBuilder::new();
        row_builder = row_builder.color(theme.color.sc);

        // 最初の行は TopLeft、それ以外は CrossLeft
        if i == 0 {
            row_builder = row_builder.str(&curved_lines.top_left);
        } else {
            row_builder = row_builder.str(&curved_lines.cross_left);
        }

        let final_prompt = row_builder
            .str(h)
            .end_color()
            .connect(left_content)
            .color(theme.color.pc)
            .str(&connection)
            .end_color()
            .connect(right_content)
            .color(theme.color.sc)
            .str(h)
            .str(if i == 0 {
                &curved_lines.top_right
            } else {
                &curved_lines.cross_right
            })
            .end_color();

        println!("{}", final_prompt.build());
    }

    // 最終行の描画
    let end = ZshPromptBuilder::new()
        .color(theme.color.sc)
        .str(&curved_lines.bottom_left)
        .str(h)
        .str(" ")
        .end_color();
    print!("{}", end.build())
}
