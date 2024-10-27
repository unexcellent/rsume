use minijinja::context;

use crate::templates::coruscant::{
    data_model::{publication::Publication, supported_resume_data::SupportedResumeData},
    shared::{entry::build_entry_single_date, render_template::render_template},
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

    for publication in publications {
        entries_html.push_str(&build_entry_single_date(
            publication.release_date.clone(),
            publication.name.clone(),
            publication.summary.clone(),
            Some(build_footer(publication)),
        ));
    }

    entries_html
}

fn build_footer(publication: &Publication) -> String {
    format!(
        "{} - Link: {}",
        publication.publisher.clone(),
        publication.url.clone()
    )
}
