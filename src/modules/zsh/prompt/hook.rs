use crate::zsh::theme::manager;

pub fn hook() {
    let theme = manager::load_theme();
    let lines_len = theme.prompt_contents_list.len();
    print!("{}", "\n".repeat(lines_len));
}
