use crate::templates::coruscant::supported_resume_data::SupportedResumeData;

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

        let email_icon = include_str!("icons/email.svg");

        let html = format!(
            "
            <div class='basics-box'>
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
                    </div>
                </div>
            </div>
            "
        );

        html.to_string()
    }
}
