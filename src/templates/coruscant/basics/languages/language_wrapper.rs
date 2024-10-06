use minijinja::context;

use crate::templates::coruscant::{
    data_model::supported_resume_data::SupportedResumeData,
    shared::render_template::render_template, supported_languages::SupportedLanguages,
};

/// Return the languages wrapper as HTML.
pub fn build_languages_wrapper(
    resume_data: &SupportedResumeData,
    language: &SupportedLanguages,
) -> String {
    if resume_data.languages.is_empty() {
        return String::new();
    }

    let percentages: Vec<u8> = resume_data
        .languages
        .iter()
        .map(|l| l.percentage().unwrap())
        .collect();

    let rendered_template = render_template(
        include_str!("index.html"),
        context!(
            title => language.languages_section_title(),
            languages => resume_data.languages,
            percentages => percentages,
        ),
    );

    match rendered_template {
        Ok(t) => t,
        Err(_) => panic!("Failed to render languages template."),
    }
}
