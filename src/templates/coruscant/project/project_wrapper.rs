use minijinja::context;

use crate::templates::coruscant::{
    data_model::{project::Project, supported_resume_data::SupportedResumeData},
    shared::{entry::build_entry_start_and_end, render_template::render_template},
    supported_languages::SupportedLanguages,
};

/// Return the project wrapper as HTML.
pub fn build_project_wrapper(
    resume_data: &SupportedResumeData,
    language: &SupportedLanguages,
) -> String {
    if resume_data.projects.is_empty() {
        return String::new();
    }

    let rendered_template = render_template(
        include_str!("index.html"),
        context!(entries => build_entries(&resume_data.projects), title => language.projects_section_title()),
    );

    match rendered_template {
        Ok(t) => t,
        Err(_) => panic!("Failed to render project template."),
    }
}

fn build_entries(projects: &Vec<Project>) -> String {
    let mut entries_html = String::new();

    for project in projects {
        entries_html.push_str(&build_entry_start_and_end(
            project.start_date.clone().unwrap_or_default(),
            project.end_date.clone().unwrap_or_default(),
            project.name.clone(),
            build_entry_body(project),
            build_footer(project),
        ));
    }

    entries_html
}

fn build_entry_body(project: &Project) -> String {
    let mut body = String::new();

    if let Some(description) = &project.description {
        body.push_str(description);
    }

    if !project.highlights.is_empty() {
        body.push_str("\n<ul>");
        for highlight in &project.highlights {
            body.push_str(&format!("\n<li>{highlight}</li>"));
        }
        body.push_str("\n</ul>");
    }

    body
}

fn build_footer(project: &Project) -> Option<String> {
    let mut parts: Vec<String> = Vec::new();

    if !project.roles.is_empty() {
        parts.push(project.roles.join(", "));
    }

    if let Some(entity) = &project.entity {
        parts.push(entity.clone());
    }

    if !project.keywords.is_empty() {
        parts.push(project.keywords.join(", "));
    }

    if let Some(url) = &project.url {
        parts.push(format!("Link: {url}"));
    }

    if parts.is_empty() {
        None
    } else {
        Some(parts.join(" - "))
    }
}
