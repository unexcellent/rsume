use json_resume::Resume;

use super::supported_resume_data::SupportedResumeData;

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
        let name = self.resume_data.basics.name.clone();
        let image = self.resume_data.basics.image.clone();
        let label = self.resume_data.basics.label.clone();

        let html = format!(
            "
<html>
    <head>
        <style>
            .root {{
                padding: 2.5%;
                padding-top: 3%;
                width: 100%;
                height: 100%;
                color: #f4f4f4;
            }}

            .basics-box {{
                width: 89%;
                height: fit-content;
                padding: 20pt;
                
                background-color: white;
                border-radius: 20pt;
                box-shadow: 0 0 50pt rgba(0, 0, 0, 0.2);
            }}

            .basics-wrapper {{
                height: 80pt;
                display: flex;
                display: -webkit-flex;
                flex-direction: row;
                justify-content: center;
                -webkit-box-align: center;
                margin: auto;
            }}

            .profile-image img {{
                width: 80pt;
                height: 80pt;
                border-radius: 50%;
                align-self: center;
                justify-self: center;
            }}

            .name-and-label {{
                align-self: center;
                justify-self: center;
            }}

            .name {{
                color: black;
                font-family: 'Verdana';
                font-size: 24pt;
                font-weight: bold;
                margin-bottom: 10pt;
            }}

            .label {{
                color: black;
                font-family: 'Verdana';
                font-size: 16pt;
            }}
            
        </style>
    </head>
    <body>
        <div class='root'>
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
                </div>
            </div>
        </div>
    </body>
</html>
        "
        );

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
