use json_resume;

/// The template requires some fields in the resume data that are optional in json_resume. These structs simplify the generation process.
#[derive(Clone, Debug)]
pub struct SupportedResumeData {
    pub basics: Basics,
    pub education: Vec<Education>,
}
impl SupportedResumeData {
    pub fn try_from(resume_data: json_resume::Resume) -> Result<Self, String> {
        Ok(Self {
            basics: match resume_data.basics {
                Some(b) => Basics::try_from(b)?,
                None => return Err("The field basics is required for this template.".to_string()),
            },
            education: resume_data
                .education
                .into_iter()
                .map(|edu| Education::try_from(edu).unwrap())
                .collect(),
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
            name: get_mandatory_field(basics.name, "basics.name")?,
            label: get_mandatory_field(basics.label, "basics.label")?,
            image: get_mandatory_field(basics.image, "basics.image")?,
            email: get_mandatory_field(basics.email, "basics.email")?,
            phone: get_mandatory_field(basics.phone, "basics.phone")?,
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
            address: get_mandatory_field(location.address, "location.address")?,
            postal_code: get_mandatory_field(location.postal_code, "location.postal_code")?,
            city: get_mandatory_field(location.city, "location.city")?,
            country_code: get_mandatory_field(location.country_code, "location.country_code")?,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Education {
    pub institution: String,
    pub start_date: String,
    pub end_date: String,
}
impl Education {
    pub fn try_from(education: json_resume::Education) -> Result<Self, String> {
        Ok(Self {
            institution: get_mandatory_field(education.institution, "education.institution")?,
            start_date: get_mandatory_field(education.start_date, "education.start_date")?,
            end_date: get_mandatory_field(education.end_date, "education.end_date")?,
        })
    }
}

fn get_mandatory_field<T>(field: Option<T>, field_name: &str) -> Result<T, String> {
    match field {
        Some(f) => Ok(f),
        None => Err(format!(
            "The field {field_name} is required for this template."
        )),
    }
}
