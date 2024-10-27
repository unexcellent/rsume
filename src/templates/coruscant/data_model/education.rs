use super::utils::get_mandatory_field;

#[derive(Clone, Debug)]
pub struct Education {
    pub institution: String,
    pub start_date: String,
    pub end_date: String,
    pub study_type: Option<String>,
    pub area: String,
    pub score: Option<String>,
}
impl Education {
    pub fn try_from(education: json_resume::Education) -> Result<Self, String> {
        Ok(Self {
            institution: get_mandatory_field(education.institution, "education.institution")?,
            start_date: get_mandatory_field(education.start_date, "education.start_date")?,
            end_date: get_mandatory_field(education.end_date, "education.end_date")?,
            study_type: education.study_type,
            area: get_mandatory_field(education.area, "education.area")?,
            score: education.score,
        })
    }
}
