use zsh_seq::ZshPromptBuilder;

use crate::prompt_theme;

pub fn hook() -> ZshPromptBuilder {
    let theme = prompt_theme();
    let lines_len = theme.prompt_contents_list.len();
    let hook = "\n".repeat(lines_len).to_owned();
    ZshPromptBuilder::new().str(&hook)
}
