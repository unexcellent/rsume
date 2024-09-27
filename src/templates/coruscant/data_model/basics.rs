use super::{location::Location, utils::get_mandatory_field};

#[derive(Clone, Debug)]
pub struct Basics {
    pub name: String,
    pub label: String,
    pub image: String,
    pub email: String,
    pub phone: String,
    pub location: Location,
}
impl Basics {
    pub fn try_from(basics: json_resume::Basics) -> Result<Self, String> {
        Ok(Self {
            name: get_mandatory_field(basics.name, "basics.name")?,
            label: get_mandatory_field(basics.label, "basics.label")?,
            image: get_mandatory_field(basics.image, "basics.image")?,
            email: get_mandatory_field(basics.email, "basics.email")?,
            phone: get_mandatory_field(basics.phone, "basics.phone")?,
            location: Location::try_from(basics.location)?,
        })
    }
}
