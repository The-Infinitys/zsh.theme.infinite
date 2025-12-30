use crate::prompt_theme;

pub fn hook() {
    let theme = prompt_theme();
    let lines_len = theme.prompt_contents_list.len();
    print!("{}", "\n".repeat(lines_len));
}
