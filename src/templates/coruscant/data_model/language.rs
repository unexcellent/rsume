use super::utils::get_mandatory_field;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Language {
    pub language: String,
    pub fluency: String,
}
impl Language {
    pub fn try_from(language: json_resume::Language) -> Result<Self, String> {
        Ok(Self {
            language: get_mandatory_field(language.language, "language.language")?,
            fluency: get_mandatory_field(language.fluency, "language.fluency")?,
        })
    }
}
