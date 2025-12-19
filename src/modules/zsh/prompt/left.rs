use crate::zsh::theme_manager;
pub fn left() {
    let theme = theme_manager::load_theme();
    print!("LEFT_PROMPT (BG: {:?}, FG: {:?})", theme.color.bg, theme.color.fg);
}
