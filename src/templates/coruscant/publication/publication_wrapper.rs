use minijinja::context;

use crate::templates::coruscant::{
    data_model::{publication::Publication, supported_resume_data::SupportedResumeData},
    shared::{entry::build_entry_start_and_end, render_template::render_template},
    supported_languages::SupportedLanguages,
};

/// Return the publication wrapper as HTML.
pub fn build_publication_wrapper(
    resume_data: &SupportedResumeData,
    language: &SupportedLanguages,
) -> String {
    if resume_data.publications.is_empty() {
        return String::new();
    }

    let rendered_template = render_template(
        include_str!("index.html"),
        context!(entries => build_entries(&resume_data.publications), title => language.publication_section_title()),
    );

    match rendered_template {
        Ok(t) => t,
        Err(_) => panic!("Failed to render publication template."),
    }
}

fn build_entries(publications: &Vec<Publication>) -> String {
    let mut entries_html = String::new();

    for entry in publications {
        entries_html.push_str(&build_entry_start_and_end(
            entry.release_date.clone(),
            entry.release_date.clone(),
            entry.name.clone(),
            entry.summary.clone(),
        ));
    }

    entries_html
}
