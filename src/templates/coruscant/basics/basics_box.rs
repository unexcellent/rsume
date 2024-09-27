use crate::templates::coruscant::data_model::supported_resume_data::SupportedResumeData;

use super::{
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
        let email = &self.resume_data.basics.email;
        let phone = &self.resume_data.basics.phone;
        let address = &self.resume_data.basics.location.address;
        let city = &self.resume_data.basics.location.city;
        let postal_code = &self.resume_data.basics.location.postal_code;
        let country_code = &self.resume_data.basics.location.country_code;
        let languages = build_languages_wrapper(&self.resume_data);
        let skills = build_skills_wrapper(&self.resume_data);

        let email_icon = include_str!("icons/email.svg");
        let phone_icon = include_str!("icons/phone.svg");
        let address_icon = include_str!("icons/address.svg");

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
                        <div class='contact-info'>
                            <div class='email-icon'>
                                {email_icon}
                            </div>
                            <div class='email'>
                                {email}
                            </div>
                            <div class='phone-icon'>
                                {phone_icon}
                            </div>
                            <div class='phone'>
                                {phone}
                            </div>
                            <div class='address-icon'>
                                {address_icon}
                            </div>
                            <div class='address'>
                                {address}, <br>
                                {postal_code} {city}, {country_code}
                            </div>
                        </div>
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
