use crate::templates::coruscant::data_model::supported_resume_data::SupportedResumeData;

use super::{
    contact_info::contact_info_wrapper::build_contact_info_wrapper,
    languages::language_wrapper::build_languages_wrapper,
    skills::skills_wrapper::build_skills_wrapper,
};

pub struct BasicsBox {
    resume_data: SupportedResumeData,
}
impl BasicsBox {
    pub fn from(resume_data: SupportedResumeData) -> Self {
        Self { resume_data }
    }

    pub fn build(&self) -> String {
        let name = &self.resume_data.basics.name;
        let image = &self.resume_data.basics.image;
        let label = &self.resume_data.basics.label;
        let contact_info = build_contact_info_wrapper(&self.resume_data);
        let languages = build_languages_wrapper(&self.resume_data);
        let skills = build_skills_wrapper(&self.resume_data);

        let html = format!(
            "
            <div class='basics-box'>
                <div class='default-box'>
                    <div class='basics-wrapper'>
                        <div class='profile-image'>
                            <img src='{image}'>
                        </div>
                        <div class='name-and-label'>
                            <div class='name'>
                                {name}
                            </div>
                            <div class='label'>
                                {label}
                            </div>
                        </div>
                        {contact_info}
                    </div>
                    <div class='skills-and-languages-wrapper'>
                        {languages}
                        {skills}
                    </div>
                </div>
            </div>
            "
        );

        html.to_string()
    }
}
