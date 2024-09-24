use json_resume;

#[allow(dead_code)]
/// The template requires some fields in the resume data that are optional in json_resume. These structs simplify the generation process.
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

#[allow(dead_code)]
pub struct Basics {
    pub name: String,
    pub label: String,
    pub image: String,
}
impl Basics {
    pub fn try_from(basics: json_resume::Basics) -> Result<Self, String> {
        Ok(Self {
            name: match basics.name {
                Some(n) => n,
                None => {
                    return Err("The field basics.name is required for this template.".to_string())
                }
            },
            label: match basics.label {
                Some(l) => l,
                None => {
                    return Err("The field basics.label is required for this template.".to_string())
                }
            },
            image: match basics.image {
                Some(n) => n,
                None => {
                    return Err("The field basics.image is required for this template.".to_string())
                }
            },
        })
    }
}
