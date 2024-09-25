/// Return the content of an optional field if it is set or return an Err if it is not.
pub fn get_mandatory_field<T>(field: Option<T>, field_name: &str) -> Result<T, String> {
    match field {
        Some(f) => Ok(f),
        None => Err(format!(
            "The field {field_name} is required for this template."
        )),
    }
}
