use crate::templates::coruscant::{
    shared::entry::Entry,
    supported_resume_data::{SupportedResumeData, Work},
};

pub struct WorkWrapper {
    resume_data: SupportedResumeData,
}
impl WorkWrapper {
    pub fn from(resume_data: SupportedResumeData) -> Self {
        Self { resume_data }
    }

    pub fn build(&self) -> String {
        if self.resume_data.work.is_empty() {
            return String::new();
        }

        let entries = build_entries(&self.resume_data.work);

        let html = format!(
            "
        <div class='section-title'>
            Experience
        </div>
        {entries}
        "
        );

        html
    }
}

fn build_entries(work: &Vec<Work>) -> String {
    let mut entries_html = String::new();

    for work_entry in work {
        let entry = Entry {
            start_date: work_entry.start_date.clone(),
            end_date: work_entry.end_date.clone(),
            title: work_entry.name.clone(),
            body: build_entry_body(work_entry),
        };
        entries_html.push_str(&entry.build());
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
