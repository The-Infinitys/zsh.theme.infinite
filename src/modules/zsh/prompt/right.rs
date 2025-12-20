use zsh_seq::ZshPromptBuilder;

use crate::zsh::{prompt::PromptCurveLine, theme_manager};

pub async fn right() {
    let theme = theme_manager::load_theme();
    let curved_lines = PromptCurveLine::from(theme.connection);
    let h = &curved_lines.horizontal;
    let b = &curved_lines.bottom_right;
    let builder = ZshPromptBuilder::new()
        .color(theme.color.sc)
        .str(h)
        .str(b)
        .end_color();

    println!("{}", builder.build());
}
