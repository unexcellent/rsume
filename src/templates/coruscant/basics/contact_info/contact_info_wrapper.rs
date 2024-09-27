use minijinja::context;

use crate::templates::coruscant::{
    data_model::supported_resume_data::SupportedResumeData,
    shared::render_template::render_template,
};

/// Return the contact info wrapper as HTML.
pub fn build_contact_info_wrapper(resume_data: &SupportedResumeData) -> String {
    let rendered_template = render_template(
        include_str!("index.html"),
        context!(
            email => resume_data.basics.email,
            phone => resume_data.basics.phone,
            address => resume_data.basics.location.address,
            city => resume_data.basics.location.city,
            postal_code => resume_data.basics.location.postal_code,
            country_code => resume_data.basics.location.country_code,
            email_icon => include_str!("icons/email.svg"),
            phone_icon => include_str!("icons/phone.svg"),
            address_icon => include_str!("icons/address.svg"),
        ),
    );

    match rendered_template {
        Ok(t) => t,
        Err(_) => panic!("Failed to render contact info template."),
    }
}
