use super::utils::get_mandatory_field;

#[derive(Clone, Debug)]
pub struct Publication {
    pub name: String,
    pub publisher: String,
    pub release_date: String,
    pub url: String,
    pub summary: String,
}
impl Publication {
    pub fn try_from(publication: json_resume::Publication) -> Result<Self, String> {
        Ok(Self {
            name: get_mandatory_field(publication.name, "publication.name")?,
            publisher: get_mandatory_field(publication.publisher, "publication.publisher")?,
            release_date: get_mandatory_field(
                publication.release_date,
                "publication.release_date",
            )?,
            url: get_mandatory_field(publication.url, "publication.url")?,
            summary: get_mandatory_field(publication.summary, "publication.summary")?,
        })
    }
}
