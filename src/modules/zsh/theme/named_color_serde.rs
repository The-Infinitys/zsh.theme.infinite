use serde::{self, Deserialize, Deserializer, Serializer};
use zsh_seq::NamedColor;

// Helper function to serialize NamedColor to a string
pub fn serialize<S>(color: &NamedColor, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // For serialization, we convert NamedColor to its Zsh string representation.
    // However, for YAML, we want a more human-readable/direct representation.
    // Let's define a specific string format for each variant.
    let s = match color {
        NamedColor::Black => "Black".to_string(),
        NamedColor::Red => "Red".to_string(),
        NamedColor::Green => "Green".to_string(),
        NamedColor::Yellow => "Yellow".to_string(),
        NamedColor::Blue => "Blue".to_string(),
        NamedColor::Magenta => "Magenta".to_string(),
        NamedColor::Cyan => "Cyan".to_string(),
        NamedColor::White => "White".to_string(),
        NamedColor::LightBlack => "LightBlack".to_string(),
        NamedColor::LightRed => "LightRed".to_string(),
        NamedColor::LightGreen => "LightGreen".to_string(),
        NamedColor::LightYellow => "LightYellow".to_string(),
        NamedColor::LightBlue => "LightBlue".to_string(),
        NamedColor::LightMagenta => "LightMagenta".to_string(),
        NamedColor::LightCyan => "LightCyan".to_string(),
        NamedColor::LightWhite => "LightWhite".to_string(),
        NamedColor::Code256(code) => format!("Code256({})", code),
        NamedColor::FullColor((r, g, b)) => format!("FullColor({},{},{})", r, g, b),
    };
    serializer.serialize_str(&s)
}

// Helper function to deserialize NamedColor from a string
pub fn deserialize<'de, D>(deserializer: D) -> Result<NamedColor, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    deserialize_from_str(&s).map_err(serde::de::Error::custom)
}

pub fn deserialize_from_str(s: &str) -> Result<NamedColor, String> {
    if s == "Black" {
        Ok(NamedColor::Black)
    } else if s == "Red" {
        Ok(NamedColor::Red)
    } else if s == "Green" {
        Ok(NamedColor::Green)
    } else if s == "Yellow" {
        Ok(NamedColor::Yellow)
    } else if s == "Blue" {
        Ok(NamedColor::Blue)
    } else if s == "Magenta" {
        Ok(NamedColor::Magenta)
    } else if s == "Cyan" {
        Ok(NamedColor::Cyan)
    } else if s == "White" {
        Ok(NamedColor::White)
    } else if s == "LightBlack" {
        Ok(NamedColor::LightBlack)
    } else if s == "LightRed" {
        Ok(NamedColor::LightRed)
    } else if s == "LightGreen" {
        Ok(NamedColor::LightGreen)
    } else if s == "LightYellow" {
        Ok(NamedColor::LightYellow)
    } else if s == "LightBlue" {
        Ok(NamedColor::LightBlue)
    } else if s == "LightMagenta" {
        Ok(NamedColor::LightMagenta)
    } else if s == "LightCyan" {
        Ok(NamedColor::LightCyan)
    } else if s == "LightWhite" {
        Ok(NamedColor::LightWhite)
    } else if s.starts_with("Code256(") && s.ends_with(')') {
        let code_str = &s[8..s.len() - 1];
        let code = code_str
            .parse::<u8>()
            .map_err(|e| format!("Invalid Code256 format: {}", e))?;
        Ok(NamedColor::Code256(code))
    } else if s.starts_with("FullColor(") && s.ends_with(')') {
        let parts: Vec<&str> = s[10..s.len() - 1].split(',').collect();
        if parts.len() == 3 {
            let r = parts[0]
                .trim()
                .parse::<u8>()
                .map_err(|e| format!("Invalid FullColor format (R): {}", e))?;
            let g = parts[1]
                .trim()
                .parse::<u8>()
                .map_err(|e| format!("Invalid FullColor format (G): {}", e))?;
            let b = parts[2]
                .trim()
                .parse::<u8>()
                .map_err(|e| format!("Invalid FullColor format (B): {}", e))?;
            Ok(NamedColor::FullColor((r, g, b)))
        } else {
            Err(format!(
                "Invalid FullColor format: {}. Expected FullColor(r,g,b)",
                s
            ))
        }
    } else {
        Err(format!(
            "Unknown NamedColor variant or invalid format: {}",
            s
        ))
    }
}
