use zsh_seq::ZshPromptBuilder;

use crate::zsh::theme::manager;

pub async fn transient(exit_code: Option<i32>) {
    let transient_str = "❯ ";
    let theme = manager::load_theme();
    let color = match exit_code {
        Some(0) => theme.transient_color.pc,
        _ => theme.transient_color.sc,
    };
    let prompt = ZshPromptBuilder::new()
        .color(color)
        .str(transient_str)
        .end_color()
        .reset_styles();
    print!("{}", prompt.build());
}
