use crate::zsh::theme_manager;
pub fn right() {
    let theme = theme_manager::load_theme();
    print!("RIGHT_PROMPT (Sep: {:?}, Conn: {:?})", theme.separation, theme.connection);
}
