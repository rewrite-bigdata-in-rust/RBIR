use std::collections::HashMap;
use tera::Value;

/// Convert given string to snake case.
///
/// - `Apache OpenDAL` => `apache_opendal`
/// - `LanceDB` => `lancedb`
pub fn to_snack_case(input: &str) -> String {
    input.to_lowercase().replace(" ", "_")
}

pub fn to_snack_case_filter(v: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    let input = v
        .as_str()
        .ok_or_else(|| tera::Error::msg("The value is not a string"))?;
    Ok(to_snack_case(input).into())
}
