use minijinja::context;

use crate::templates::coruscant::{
    data_model::supported_resume_data::SupportedResumeData,
    shared::render_template::render_template,
};

/// Return the skills wrapper as HTML.
pub fn build_skills_wrapper(resume_data: &SupportedResumeData) -> String {
    if resume_data.skills.is_empty() {
        return String::new();
    }

    let rendered_template = render_template(
        include_str!("index.html"),
        context!(skills => resume_data.skills),
    );

    match rendered_template {
        Ok(t) => t,
        Err(_) => panic!("Failed to render skills template."),
    }
}
