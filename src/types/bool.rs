use super::ParserError;

pub fn parse_bool(value: &str) -> Result<bool, ParserError> {
    match value.trim() {
        "true" | "True" | "TRUE" | "t" | "T" | "1" => Ok(true),
        "false" | "False" | "FALSE" | "f" | "F" | "0" => Ok(false),
        _ => Err(ParserError::InvalidBool),
    }
}
