use crate::zsh::theme_manager;
use zsh_seq::{NamedColor, ZshPromptBuilder};

pub async fn transient(exit_code: Option<i32>) {
    let theme = theme_manager::load_theme();
    let transient_str = "❯ ";

    let color = match exit_code {
        Some(0) => theme.color.pc,
        _ => NamedColor::Red,
    };

    let prompt = ZshPromptBuilder::new()
        .color(color)
        .str(transient_str)
        .end_color()
        .reset_styles();

    print!("{}", prompt.build());
}
