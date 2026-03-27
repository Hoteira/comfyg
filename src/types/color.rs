use super::ParserError;

pub fn parse_color(value: &str) -> Result<[u8; 4], ParserError> {
    let value = value.trim();

    if let Some(hex) = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
        .or_else(|| value.strip_prefix('#'))
    {
        return parse_hex_color(hex);
    }

    if let Some(inner) = value
        .strip_prefix("rgba(")
        .and_then(|s| s.strip_suffix(')'))
    {
        return parse_rgb_color(inner);
    }

    if let Some(inner) = value.strip_prefix("rgb(").and_then(|s| s.strip_suffix(')')) {
        return parse_rgb_color(inner);
    }

    parse_hex_color(value)
}

fn parse_rgb_color(value: &str) -> Result<[u8; 4], ParserError> {
    let mut parts = value.split(',');

    let mut next_u8 = || -> Option<u8> { parts.next()?.trim().parse().ok() };

    let r = next_u8().ok_or(ParserError::InvalidColor)?;
    let g = next_u8().ok_or(ParserError::InvalidColor)?;
    let b = next_u8().ok_or(ParserError::InvalidColor)?;
    // alpha is optional, defaults to opaque
    let a = next_u8().unwrap_or(0xFF);

    // reject anything with extra components
    if parts.next().is_some() {
        return Err(ParserError::InvalidColor);
    }

    Ok([r, g, b, a])
}

fn parse_hex_color(value: &str) -> Result<[u8; 4], ParserError> {
    let parse = |s: &str| u8::from_str_radix(s, 16).map_err(|_| ParserError::InvalidColor);

    match value.len() {
        6 => Ok([
            parse(&value[0..2])?,
            parse(&value[2..4])?,
            parse(&value[4..6])?,
            255,
        ]),
        8 => Ok([
            parse(&value[0..2])?,
            parse(&value[2..4])?,
            parse(&value[4..6])?,
            parse(&value[6..8])?,
        ]),
        _ => Err(ParserError::InvalidColor),
    }
}
