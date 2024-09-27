use crate::templates::coruscant::data_model::supported_resume_data::SupportedResumeData;

pub struct SkillsWrapper {
    resume_data: SupportedResumeData,
}
impl SkillsWrapper {
    pub fn from(resume_data: SupportedResumeData) -> Self {
        Self { resume_data }
    }

    pub fn build(&self) -> String {
        if self.resume_data.skills.is_empty() {
            return String::new();
        }

        let entries = build_entries(&self.resume_data.skills);

        let html = format!(
            "
            <div class='skills-title'> Skills </div>
            {entries}
        "
        );

        html
    }
}

fn build_entries(skills: &Vec<String>) -> String {
    let mut html = String::new();

    for skill in skills {
        html.push_str(&format!(
            "
            <div class='skill'>{skill}</div>
        "
        ));
    }

    html
}
