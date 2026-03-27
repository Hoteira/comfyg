use super::ParserError;

pub fn parse_color(value: &str) -> Result<[u8; 4], ParserError> {
    let color_string = String::from(value.replace(" ", ""));

    //formats:
    // #RRGGBBAA V
    // RRGGBBAA V
    // RRGGBB V
    // rgb(r, g, b) V
    // rgba(r, g, b, a) V
    // 0xRRGGBBAA V
    // 0XRRGGBBAA V

    if let Some(hex_val) = color_string
        .strip_prefix("0x")
        .or_else(|| color_string.strip_prefix("0X"))
        .or_else(|| color_string.strip_prefix("#"))
    {
        return parse_hex_color(hex_val);
    }

    if let Some(rgb_val) = color_string.strip_prefix("rgba(") {
        let inner = rgb_val.trim_end_matches(')');
        return parse_rgb_color(inner);
    }

    if let Some(rgb_val) = color_string.strip_prefix("rgb(") {
        let inner = rgb_val.trim_end_matches(')');
        return parse_rgb_color(inner);
    }

    parse_hex_color(&color_string)
}

fn parse_rgb_color(value: &str) -> Result<[u8; 4], ParserError> {
    let rgb = value.split(",").map(|s| s.trim()).collect::<Vec<_>>();
    let err = ParserError::InvalidColor(format!("Invalid rgb color: {}", value));

    if rgb.len() == 3 {
        return Ok([
            u8::from_str_radix(rgb[0], 10).map_err(|_| err.clone())?,
            u8::from_str_radix(rgb[1], 10).map_err(|_| err.clone())?,
            u8::from_str_radix(rgb[2], 10).map_err(|_| err.clone())?,
            0xFF,
        ]);
    } else if rgb.len() == 4 {
        return Ok([
            u8::from_str_radix(rgb[0], 10).map_err(|_| err.clone())?,
            u8::from_str_radix(rgb[1], 10).map_err(|_| err.clone())?,
            u8::from_str_radix(rgb[2], 10).map_err(|_| err.clone())?,
            u8::from_str_radix(rgb[3], 10).map_err(|_| err.clone())?,
        ]);
    } else {
        return Err(err);
    }
}

fn parse_hex_color(value: &str) -> Result<[u8; 4], ParserError> {
    let err = || ParserError::InvalidColor(format!("Invalid hex color: {}", value));

    match value.len() {
        6 => {
            // RRGGBB
            let r = u8::from_str_radix(&value[0..2], 16).map_err(|_| err())?;
            let g = u8::from_str_radix(&value[2..4], 16).map_err(|_| err())?;
            let b = u8::from_str_radix(&value[4..6], 16).map_err(|_| err())?;
            Ok([r, g, b, 255])
        }
        8 => {
            // RRGGBBAA
            let r = u8::from_str_radix(&value[0..2], 16).map_err(|_| err())?;
            let g = u8::from_str_radix(&value[2..4], 16).map_err(|_| err())?;
            let b = u8::from_str_radix(&value[4..6], 16).map_err(|_| err())?;
            let a = u8::from_str_radix(&value[6..8], 16).map_err(|_| err())?;
            Ok([r, g, b, a])
        }
        _ => Err(err()),
    }
}
