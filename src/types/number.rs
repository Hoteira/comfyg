use super::ParserError;

pub fn parse_number(value: &str) -> Result<f64, ParserError> {
    let value = value.trim();

    if let Some(hex) = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
    {
        return u64::from_str_radix(hex, 16)
            .map(|v| v as f64)
            .map_err(|_| ParserError::InvalidNumber);
    }

    value.parse::<f64>().map_err(|_| ParserError::InvalidNumber)
}
