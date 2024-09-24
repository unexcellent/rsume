use json_resume::Resume;

pub struct Coruscant {
    _resume_data: Resume,
}

impl Coruscant {
    pub fn new(json_resume_data: Resume) -> Self {
        Coruscant {
            _resume_data: json_resume_data,
        }
    }

    /// Build the resume as printable HTML.
    pub fn build(&self) -> String {
        let html = "
            <html>
                <head>
                    <title>\"Hello world\"</title>
                </head>
                <body>
                    <h1>\"Hello world\"</h1>
                </body>
            </html>
        ";

        html.to_string()
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

        let generated_html = Coruscant::new(load_json_resume(&resume_data_path).unwrap()).build();

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
