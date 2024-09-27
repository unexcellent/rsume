use crate::Languages;

/// Languages supported by this template.
pub enum SupportedLanguages {
    EN,
    DE,
}
impl SupportedLanguages {
    pub fn try_from(language: &Languages) -> Result<Self, String> {
        match language {
            Languages::EN => Ok(Self::EN),
            Languages::DE => Ok(Self::DE),
        }
    }

    pub fn work_section_title(&self) -> String {
        match self {
            SupportedLanguages::EN => "Experience".to_string(),
            SupportedLanguages::DE => "Arbeitserfahrung".to_string(),
        }
    }
}
