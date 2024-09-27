use json_resume::Resume;
use minijinja::context;

use super::{
    basics::basics_box::build_basics_wrapper,
    data_model::supported_resume_data::SupportedResumeData,
    education::education_wrapper::build_education_wrapper,
    shared::render_template::render_template, work::work_wrapper::build_work_wrapper,
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
        let rendered_template = render_template(
            include_str!("index.html"),
            context!(
                style => include_str!("style.css"),
                basics => build_basics_wrapper(&self.resume_data),
                education => build_education_wrapper(&self.resume_data),
                work => build_work_wrapper(&self.resume_data),
            ),
        );

        match rendered_template {
            Ok(t) => t,
            Err(_) => panic!("Failed to render root template."),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use std::{fs, path::PathBuf};

    use crate::{generate_pdf, io::load_json_resume::load_json_resume, templates::Coruscant};

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
