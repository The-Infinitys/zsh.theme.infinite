use crate::zsh::{
    prompt::{Prompt, PromptConnection, PromptCurveLine},
    theme_manager,
};
use crossterm::terminal;
use futures::future::join_all;
use unicode_width::UnicodeWidthStr;
use zsh_seq::ZshPromptBuilder;

pub async fn left() {
    let mut prompt = Prompt::default();
    let curved_lines = PromptCurveLine::default();
    let l = PromptConnection::Line.to_string();
    let theme = theme_manager::load_theme();

    // 左右のプロンプトコンテンツの各項目を並列で取得
    let left_futures: Vec<_> = theme
        .prompt_contents
        .left
        .iter()
        .map(|content_type| async move { content_type.content().await })
        .collect();

    let right_futures: Vec<_> = theme
        .prompt_contents
        .right
        .iter()
        .map(|content_type| async move { content_type.content().await })
        .collect();

    let (left_contents_results, right_contents_results) =
        tokio::join!(join_all(left_futures), join_all(right_futures));

    // Prompt構造体に追加
    for content in left_contents_results.into_iter().flatten() {
        prompt.add_left(&content);
    }
    for content in right_contents_results.into_iter().flatten() {
        prompt.add_right(&content);
    }

    let left_content = prompt.render_left(&theme);
    let right_content = prompt.render_right(&theme);

    let terminal_width = terminal::size().map(|(w, _)| w).unwrap_or(80) as usize;

    // ZshPromptBuilderが生成する文字列はエスケープシーケンスを含んでいるため、
    // 表示される幅を正確に測定するには、エスケープシーケンスを除外する必要があります。
    // zsh_seq::ZshPromptBuilder.build() はエスケープシーケンスを除外しないので、
    // ここでエスケープシーケンスを考慮した幅を測定する必要があります。
    // 仮に、エスケープシーケンスを含まない純粋な文字列として幅を測定します。
    // 実際にはzsh_seqクレート側で表示幅を計算する関数が必要になるかもしれません。
    // ここでは単純にエスケープシーケンスを無視して文字幅を測定します。
    let left_width = UnicodeWidthStr::width(left_content.text().as_str());
    let right_width = UnicodeWidthStr::width(right_content.text().as_str());

    // カーブの線の部分とスペースの幅も考慮に入れる
    let fixed_width = 4;
    let connection_len = terminal_width.saturating_sub(left_width + right_width + fixed_width);

    let connection = theme.connection.to_string().repeat(connection_len);

    let final_prompt = ZshPromptBuilder::new()
        .color(theme.color.sc)
        .str(&curved_lines.top_left)
        .str(&l)
        .end_color()
        .connect(left_content)
        .color(theme.color.pc)
        .str(&connection)
        .end_color()
        .connect(right_content)
        .color(theme.color.sc)
        .str(&l)
        .str(&curved_lines.top_right)
        .end_color()
        .newline()
        .color(theme.color.sc)
        .str(&curved_lines.bottom_left)
        .str(&l)
        .str(" ")
        .end_color();
    print!("{}", final_prompt.build());
}
