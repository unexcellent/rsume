use minijinja::context;

use crate::templates::coruscant::{
    data_model::{education::Education, supported_resume_data::SupportedResumeData},
    shared::{entry::build_entry, render_template::render_template},
};

/// Return the education wrapper as HTML.
pub fn build_education_wrapper(resume_data: &SupportedResumeData) -> String {
    if resume_data.education.is_empty() {
        return String::new();
    }

    let rendered_template = render_template(
        include_str!("index.html"),
        context!(entries => build_entries(&resume_data.education)),
    );

    match rendered_template {
        Ok(t) => t,
        Err(_) => panic!("Failed to render work template."),
    }
}

fn build_entries(education: &Vec<Education>) -> String {
    let mut entries_html = String::new();

    for education_entry in education {
        entries_html.push_str(&build_entry(
            education_entry.start_date.clone(),
            education_entry.end_date.clone(),
            education_entry.institution.clone(),
            build_entry_body(education_entry),
        ));
    }

    entries_html
}

fn build_entry_body(education_entry: &Education) -> String {
    let study_type = match education_entry.clone().study_type {
        None => "".to_string(),
        Some(s) => format!("{s} in "),
    };
    let area = education_entry.clone().area;

    format!("{study_type}{area}")
}
