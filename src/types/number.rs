use crate::types::ParserError;

pub fn parse_number(value: &str) -> Result<f64, ParserError> {
    let n = value.replace(" ", "");
    let err = ParserError::InvalidNumber(format!("Invalid number: {}", value));
    let hex_err = ParserError::InvalidNumber(format!("Invalid number: {}", value));

    if value.starts_with("0x") {
        let n = n.trim_start_matches("0x");
        let val = u64::from_str_radix(n, 16).map_err(|_| hex_err)?;
        return Ok(val as f64);
    }

    if value.starts_with("0X") {
        let n = n.trim_start_matches("0X");
        let val = u64::from_str_radix(n, 16).map_err(|_| hex_err)?;
        return Ok(val as f64);
    }

    Ok(n.parse::<f64>().map_err(|_| err)?)
}
