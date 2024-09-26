use crate::templates::coruscant::data_model::{
    language::Language, supported_resume_data::SupportedResumeData,
};

pub struct LanguageWrapper {
    resume_data: SupportedResumeData,
}
impl LanguageWrapper {
    pub fn from(resume_data: SupportedResumeData) -> Self {
        Self { resume_data }
    }

    pub fn build(&self) -> String {
        if self.resume_data.languages.is_empty() {
            return String::new();
        }

        let entries = build_entries(&self.resume_data.languages);

        let html = format!(
            "
            <div class='languages-title'> Languages </div>
            {entries}
        "
        );

        html
    }
}

fn build_entries(languages: &Vec<Language>) -> String {
    let mut html = String::new();

    for language_entry in languages {
        let language_name = language_entry.language.clone();
        let language_fluency = language_entry.fluency.clone();
        let language_percentage = language_entry.percentage();

        match language_percentage {
            Ok(percentage) => html.push_str(&format!(
                "
                <div class='language-name'>{language_name}</div>
                <div class='progress-bar'>
                    <div class='progress' style='width: {percentage}%;'></div>
                </div>
                <div class='language-fluency'>{language_fluency}</div>
            "
            )),
            Err(_) => html.push_str(&format!(
                "<div class='language-name'>{language_name} ({language_fluency})</div>"
            )),
        }
    }

    html
}
