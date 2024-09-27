use minijinja::context;

use crate::templates::coruscant::shared::render_template::render_template;

/// Return an entry as HTML. Entries are the box
pub fn build_entry(start_date: String, end_date: String, title: String, body: String) -> String {
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
        Err(_) => panic!("Failed to render contact info template."),
    }
}
