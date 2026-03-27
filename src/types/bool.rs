use super::ParserError;

pub fn parse_bool(value: &str) -> Result<bool, ParserError> {
    match value.replace(" ", "").to_lowercase().as_str() {
        "true" | "t" | "1" => Ok(true),
        "false" | "f" | "0" => Ok(false),
        _ => Err(ParserError::InvalidBool(value.to_string())),
    }
}
