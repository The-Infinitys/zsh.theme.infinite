use zsh_seq::ZshPromptBuilder;

use crate::zsh::{
    prompt::{Prompt, PromptConnection, PromptCurveLine},
    theme_manager,
};
pub fn left() {
    let prompt = Prompt::default();
    let curved_lines = PromptCurveLine::default();
    let l = PromptConnection::Line.to_string();
    let theme = theme_manager::load_theme();
    let left_contents = prompt.render_left(&theme);
    let right_contents = prompt.render_right(&theme);
    let connection = "";
    let prompt = ZshPromptBuilder::new()
        .color(theme.color.sc)
        .str(&curved_lines.top_left)
        .str(&l)
        .end_color()
        .connect(left_contents)
        .color(theme.color.pc)
        .str(connection)
        .end_color()
        .connect(right_contents)
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
    print!("{}", prompt.build());
}
