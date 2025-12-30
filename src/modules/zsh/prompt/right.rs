use zsh_seq::{NamedColor, ZshPromptBuilder};

use crate::{
    prompt_theme,
    zsh::prompt::{PromptConnection, PromptCurveLine},
};

pub async fn right() {
    let theme = prompt_theme();
    let (sc, connection) = match theme.prompt_contents_list.last() {
        Some(contents) => (contents.color.sc, contents.connection),
        None => (NamedColor::LightBlack, PromptConnection::default()),
    };
    let curved_lines = PromptCurveLine::from(connection);
    let h = &curved_lines.horizontal;
    let end = ZshPromptBuilder::new()
        .color(sc)
        .str(h)
        .str(&curved_lines.bottom_right)
        .end_color();
    print!("{}", end.build())
}
