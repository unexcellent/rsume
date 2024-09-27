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
}
