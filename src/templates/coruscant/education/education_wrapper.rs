use crate::templates::coruscant::{
    shared::entry::Entry,
    supported_resume_data::{Education, SupportedResumeData},
};

pub struct EducationWrapper {
    resume_data: SupportedResumeData,
}
impl EducationWrapper {
    pub fn from(resume_data: SupportedResumeData) -> Self {
        Self { resume_data }
    }

    pub fn build(&self) -> String {
        if self.resume_data.education.is_empty() {
            return String::new();
        }

        let entries = build_entries(&self.resume_data.education);

        let html = format!(
            "
        <div class='section-title'>
            Education
        </div>
        {entries}
        "
        );

        html
    }
}

fn build_entries(education: &Vec<Education>) -> String {
    let mut entries_html = String::new();

    for education_entry in education {
        let entry = Entry {
            start_date: education_entry.start_date.clone(),
            end_date: education_entry.end_date.clone(),
            title: education_entry.institution.clone(),
            body: "IF YOU SEE THIS THEN A FIELD STILL NEEDS TO BE SET".to_string(),
        };
        entries_html.push_str(&entry.build());
    }

    entries_html
}
