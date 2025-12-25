use crate::zsh::theme_manager;

pub fn hook() {
    let theme = theme_manager::load_theme();
    let lines_len = theme.prompt_contents_list.len();
    print!("{}", "\n".repeat(lines_len));
}
