use zsh_seq::ZshPromptBuilder;

use crate::prompt_theme;

pub async fn transient(exit_code: &Option<i32>) -> ZshPromptBuilder {
    let transient_str = "❯ ";
    let theme = prompt_theme();
    let color = match exit_code {
        Some(0) => theme.transient_color.pc,
        _ => theme.transient_color.sc,
    };
    ZshPromptBuilder::new()
        .color(color)
        .str(transient_str)
        .end_color()
        .reset_styles()
}
