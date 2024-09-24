use json_resume::Resume;

use super::{
    basics::basics_box::BasicsBox, education::education_wrapper::EducationWrapper,
    supported_resume_data::SupportedResumeData,
};

#[allow(dead_code)]
pub struct Coruscant {
    resume_data: SupportedResumeData,
}

impl Coruscant {
    pub fn try_from(json_resume_data: Resume) -> Result<Self, String> {
        Ok(Coruscant {
            resume_data: SupportedResumeData::try_from(json_resume_data)?,
        })
    }

    /// Build the resume as printable HTML.
    pub fn build(&self) -> String {
        let style = include_str!("style.css");

        let basics = BasicsBox::from(self.resume_data.clone()).build();
        let education = EducationWrapper::from(self.resume_data.clone()).build();

        let html = format!(
            "
<html>
    <head>
        <style>{style}</style>
    </head>
    <body style='background-color: #F4F4F4;'>
        <div class='root'>
            {basics}
            {education}
        </div>
    </body>
</html>
        "
        );

        html
    }
}

#[cfg(test)]
pub mod tests {
    use std::{fs, path::PathBuf};

    use crate::{generate_pdf, load_json_resume::load_json_resume, templates::Coruscant};

    #[test]
    fn build_example() {
        let resume_data_path = PathBuf::from("examples/kirk_resume.yaml");
        let target_path = PathBuf::from("examples/coruscant.pdf");
        let html_target_path = resume_data_path.parent().unwrap().join("coruscant.html");

        let generated_html = Coruscant::try_from(load_json_resume(&resume_data_path).unwrap())
            .unwrap()
            .build();

        if let Ok(previous_html) = fs::read(&html_target_path) {
            if Vec::<u8>::from(generated_html.clone()) == previous_html {
                return;
            }
        }

        fs::write(html_target_path, generated_html).unwrap();

        let _ = fs::remove_file(&target_path);
        assert!(!target_path.is_file());

        generate_pdf(resume_data_path, target_path.clone());
        assert!(target_path.is_file());
    }
}
