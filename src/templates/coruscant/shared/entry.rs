use minijinja::context;

use crate::templates::coruscant::shared::render_template::render_template;

/// Return an entry with select start and end dates as HTML. Entries are the boxes that appear in sections like work or education.
pub fn build_entry_start_and_end(
    start_date: String,
    end_date: String,
    title: String,
    body: String,
) -> String {
    let rendered_template = render_template(
        include_str!("index.html"),
        context!(
            start_date => start_date,
            end_date => end_date,
            title => title,
            body => body,
        ),
    );

    match rendered_template {
        Ok(t) => t,
        Err(_) => panic!("Failed to render entry template."),
    }
}

/// Return an entry with a singular date as HTML. Entries are the boxes that appear in sections like work or education.
pub fn build_entry_single_date(
    date: String,
    title: String,
    body: String,
    footer: Option<String>,
) -> String {
    let rendered_template = render_template(
        include_str!("index.html"),
        context!(
            date => date,
            title => title,
            body => body,
            footer => footer,
        ),
    );

    match rendered_template {
        Ok(t) => t,
        Err(_) => panic!("Failed to render entry template."),
    }
}
