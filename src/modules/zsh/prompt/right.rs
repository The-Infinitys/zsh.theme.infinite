use zsh_seq::{NamedColor, ZshPromptBuilder};

use crate::zsh::{
    prompt::{PromptConnection, PromptCurveLine},
    theme::manager,
};

pub async fn right() {
    let theme = manager::load_theme();
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
