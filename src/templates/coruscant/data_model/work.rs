use super::utils::get_mandatory_field;

#[derive(Clone, Debug)]
pub struct Work {
    pub name: String,
    pub description: String,
    pub start_date: String,
    pub end_date: String,
    pub highlights: Vec<String>,
}
impl Work {
    pub fn try_from(work: json_resume::Work) -> Result<Self, String> {
        Ok(Self {
            name: get_mandatory_field(work.name, "education.institution")?,
            description: get_mandatory_field(work.description, "education.institution")?,
            start_date: get_mandatory_field(work.start_date, "education.start_date")?,
            end_date: get_mandatory_field(work.end_date, "education.end_date")?,
            highlights: work.highlights.iter().map(|h| h.to_string()).collect(),
        })
    }
}
