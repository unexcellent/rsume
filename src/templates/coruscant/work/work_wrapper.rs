use minijinja::context;

use crate::templates::coruscant::{
    data_model::{supported_resume_data::SupportedResumeData, work::Work},
    shared::{entry::build_entry_start_and_end, render_template::render_template},
    supported_languages::SupportedLanguages,
};

/// Return the work wrapper as HTML.
pub fn build_work_wrapper(
    resume_data: &SupportedResumeData,
    language: &SupportedLanguages,
) -> String {
    if resume_data.work.is_empty() {
        return String::new();
    }

    let rendered_template = render_template(
        include_str!("index.html"),
        context!(entries => build_entries(&resume_data.work), title => language.work_section_title()),
    );

    match rendered_template {
        Ok(t) => t,
        Err(_) => panic!("Failed to render work template."),
    }
}

fn build_entries(work: &Vec<Work>) -> String {
    let mut entries_html = String::new();

    for work_entry in work {
        entries_html.push_str(&build_entry_start_and_end(
            work_entry.start_date.clone(),
            work_entry.end_date.clone(),
            work_entry.name.clone(),
            build_entry_body(work_entry),
            None,
        ));
    }

    entries_html
}

fn build_entry_body(work_entry: &Work) -> String {
    let mut entry_body = String::new();
    entry_body.push_str(&work_entry.description);
    entry_body.push_str("\n<ul>");

    for highlight in &work_entry.highlights {
        entry_body.push_str(&format!(
            "
            \n<li>{highlight}</li>
            "
        ));
    }

    entry_body.push_str("\n</ul>");

    entry_body
}
