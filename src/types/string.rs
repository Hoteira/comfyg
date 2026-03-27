use super::ParserError;

pub fn parse_string(value: &str) -> Result<String, ParserError> {
    Ok(String::from(value))
}
