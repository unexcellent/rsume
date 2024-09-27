use minijinja::{context, Environment};

use crate::templates::coruscant::data_model::supported_resume_data::SupportedResumeData;

/// Return the skills wrapper as HTML.
pub fn build_skills_wrapper(resume_data: &SupportedResumeData) -> String {
    if resume_data.skills.is_empty() {
        return String::new();
    }

    let mut environment = Environment::new();
    environment
        .add_template("skills", include_str!("index.html"))
        .unwrap();

    let template = environment.get_template("skills").unwrap();

    template
        .render(context!(skills => resume_data.skills))
        .unwrap()
}
