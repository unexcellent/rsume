use super::utils::get_mandatory_field;

#[derive(Clone, Debug)]
pub struct Project {
    pub name: String,
    pub description: Option<String>,
    pub highlights: Vec<String>,
    pub keywords: Vec<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub url: Option<String>,
    pub roles: Vec<String>,
    pub entity: Option<String>,
}
impl Project {
    pub fn try_from(project: json_resume::Project) -> Result<Self, String> {
        Ok(Self {
            name: get_mandatory_field(project.name, "project.name")?,
            description: project.description,
            highlights: project.highlights.iter().map(|h| h.to_string()).collect(),
            keywords: project.keywords.iter().map(|k| k.to_string()).collect(),
            start_date: project.start_date,
            end_date: project.end_date,
            url: project.url,
            roles: project.roles.iter().map(|r| r.to_string()).collect(),
            entity: project.entity,
        })
    }
}
