use super::ParserError;

pub fn parse_string(value: &str) -> Result<String, ParserError> {
    Ok(value.to_string())
}
