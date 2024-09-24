use json_resume;

/// The template requires some fields in the resume data that are optional in json_resume. These structs simplify the generation process.
#[derive(Clone, Debug)]
pub struct SupportedResumeData {
    pub basics: Basics,
}
impl SupportedResumeData {
    pub fn try_from(resume_data: json_resume::Resume) -> Result<Self, String> {
        Ok(Self {
            basics: match resume_data.basics {
                Some(b) => Basics::try_from(b)?,
                None => return Err("The field basics is required for this template.".to_string()),
            },
        })
    }
}

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
            name: match basics.name {
                Some(s) => s,
                None => {
                    return Err("The field basics.name is required for this template.".to_string())
                }
            },
            label: match basics.label {
                Some(s) => s,
                None => {
                    return Err("The field basics.label is required for this template.".to_string())
                }
            },
            image: match basics.image {
                Some(s) => s,
                None => {
                    return Err("The field basics.image is required for this template.".to_string())
                }
            },
            email: match basics.email {
                Some(s) => s,
                None => {
                    return Err("The field basics.email is required for this template.".to_string())
                }
            },
            phone: match basics.phone {
                Some(s) => s,
                None => {
                    return Err("The field basics.phone is required for this template.".to_string())
                }
            },
            location: Location::try_from(basics.location)?,
        })
    }
}

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
            address: match location.address {
                Some(s) => s,
                None => {
                    return Err(
                        "The field location.address is required for this template.".to_string()
                    )
                }
            },
            postal_code: match location.postal_code {
                Some(s) => s,
                None => {
                    return Err(
                        "The field location.postal_code is required for this template.".to_string(),
                    )
                }
            },
            city: match location.city {
                Some(s) => s,
                None => {
                    return Err("The field location.city is required for this template.".to_string())
                }
            },
            country_code: match location.country_code {
                Some(s) => s,
                None => {
                    return Err(
                        "The field location.country_code is required for this template."
                            .to_string(),
                    )
                }
            },
        })
    }
}
