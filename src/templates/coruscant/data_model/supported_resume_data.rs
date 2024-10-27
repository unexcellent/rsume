use super::{
    basics::Basics, education::Education, language::Language, publication::Publication,
    utils::get_mandatory_field, work::Work,
};

/// The template requires some fields in the resume data that are optional in json_resume. These structs simplify the generation process.
#[derive(Clone, Debug)]
pub struct SupportedResumeData {
    pub basics: Basics,
    pub languages: Vec<Language>,
    pub skills: Vec<String>,
    pub work: Vec<Work>,
    pub education: Vec<Education>,
    pub publications: Vec<Publication>,
}
impl SupportedResumeData {
    pub fn try_from(resume_data: json_resume::Resume) -> Result<Self, String> {
        Ok(Self {
            basics: match resume_data.basics {
                Some(b) => Basics::try_from(b)?,
                None => return Err("The field basics is required for this template.".to_string()),
            },
            languages: resume_data
                .languages
                .into_iter()
                .map(|l| Language::try_from(l).unwrap())
                .collect(),
            skills: resume_data
                .skills
                .into_iter()
                .map(|skill| get_mandatory_field(skill.name, "skill.name").unwrap())
                .collect(),
            work: resume_data
                .work
                .into_iter()
                .map(|w| Work::try_from(w).unwrap())
                .collect(),
            education: resume_data
                .education
                .into_iter()
                .map(|edu| Education::try_from(edu).unwrap())
                .collect(),
            publications: resume_data
                .publications
                .into_iter()
                .map(|publication| Publication::try_from(publication).unwrap())
                .collect(),
        })
    }
}
