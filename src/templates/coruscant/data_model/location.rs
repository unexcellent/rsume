use super::utils::get_mandatory_field;

#[derive(Clone, Debug)]
pub struct Location {
    pub address: String,
    pub postal_code: String,
    pub city: String,
    pub country_code: String,
}
impl Location {
    pub fn try_from(location: json_resume::Location) -> Result<Self, String> {
        Ok(Self {
            address: get_mandatory_field(location.address, "location.address")?,
            postal_code: get_mandatory_field(location.postal_code, "location.postal_code")?,
            city: get_mandatory_field(location.city, "location.city")?,
            country_code: get_mandatory_field(location.country_code, "location.country_code")?,
        })
    }
}
