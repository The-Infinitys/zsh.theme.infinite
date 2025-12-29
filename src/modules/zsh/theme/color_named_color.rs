pub trait ToNamedColor {
    fn to_named_color(&self) -> zsh_seq::NamedColor;
}

impl ToNamedColor for zsh_prompts::Color {
    fn to_named_color(&self) -> zsh_seq::NamedColor {
        match self {
            Self::Black => zsh_seq::NamedColor::Black,
            Self::Red => zsh_seq::NamedColor::Red,
            Self::Green => zsh_seq::NamedColor::Green,
            Self::Yellow => zsh_seq::NamedColor::Yellow,
            Self::Blue => zsh_seq::NamedColor::Blue,
            Self::Magenta => zsh_seq::NamedColor::Magenta,
            Self::Cyan => zsh_seq::NamedColor::Cyan,
            Self::White => zsh_seq::NamedColor::White,
            Self::Rgb(r, g, b) => zsh_seq::NamedColor::FullColor((*r, *g, *b)),
        }
    }
}
