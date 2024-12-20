use crate::GloballySupportedLanguages;

/// Languages supported by this template.
pub enum SupportedLanguages {
    EN,
    DE,
}
impl SupportedLanguages {
    pub fn try_from(language: &GloballySupportedLanguages) -> Result<Self, String> {
        match language {
            GloballySupportedLanguages::EN => Ok(Self::EN),
            GloballySupportedLanguages::DE => Ok(Self::DE),
        }
    }

    pub fn languages_section_title(&self) -> String {
        match self {
            SupportedLanguages::EN => "Languages".to_string(),
            SupportedLanguages::DE => "Sprachen".to_string(),
        }
    }

    pub fn skills_section_title(&self) -> String {
        match self {
            SupportedLanguages::EN => "Skills".to_string(),
            SupportedLanguages::DE => "Kenntnisse".to_string(),
        }
    }

    pub fn work_section_title(&self) -> String {
        match self {
            SupportedLanguages::EN => "Experience".to_string(),
            SupportedLanguages::DE => "Arbeitserfahrung".to_string(),
        }
    }

    pub fn education_section_title(&self) -> String {
        match self {
            SupportedLanguages::EN => "Education".to_string(),
            SupportedLanguages::DE => "Bildung".to_string(),
        }
    }

    pub fn publication_section_title(&self) -> String {
        match self {
            SupportedLanguages::EN => "Publications".to_string(),
            SupportedLanguages::DE => "Veröffentlichungen".to_string(),
        }
    }
}
