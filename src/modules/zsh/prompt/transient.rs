use zsh_seq::{NamedColor, ZshPromptBuilder};

pub async fn transient(exit_code: Option<i32>) {
    let transient_str = "❯ ";

    let color = match exit_code {
        Some(0) => NamedColor::Green,
        _ => NamedColor::Red,
    };

    let prompt = ZshPromptBuilder::new()
        .color(color)
        .str(transient_str)
        .end_color()
        .reset_styles();

    print!("{}", prompt.build());
}
