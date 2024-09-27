mod io;
mod templates;

use std::error::Error;
use std::path::PathBuf;

use crate::io::load_json_resume::load_json_resume;
use crate::io::resolve_image_path::resolve_image_path;
use crate::io::save_to_pdf::save_to_pdf;
use crate::templates::Coruscant;
use clap::Parser;

/// Program for generating a resume from JSONResume data.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to the data describing your resume. It needs to comply with theJSONResume schema (see https://jsonresume.org/).
    resume_data_path: PathBuf,

    /// Location where the generated PDF should be stored.
    target_path: PathBuf,

    /// Language of the template. Available options are 'english' and 'deutsch'. Default is english.
    language: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let language = match args.language {
        Some(language_string) => Languages::try_from(language_string)?,
        None => Languages::EN,
    };

    generate_pdf(args.resume_data_path, args.target_path, language)?;

    Ok(())
}

/// Generate a resume and save it as a PDF.
pub fn generate_pdf(
    resume_data_path: PathBuf,
    target_path: PathBuf,
    language: Languages,
) -> Result<(), Box<dyn Error>> {
    let mut resume_data = load_json_resume(&resume_data_path).unwrap();

    if let Some(ref mut basics) = resume_data.basics {
        basics.image = resolve_image_path(&resume_data_path, &basics.image);
    }

    let template = Coruscant::try_from(resume_data, &language).unwrap();

    let html_resume = template.build();
    save_to_pdf(html_resume, &target_path)?;

    Ok(())
}

/// Language of the template
pub enum Languages {
    /// English
    EN,
    /// German
    DE,
}
impl Languages {
    pub fn try_from(language_string: String) -> Result<Self, String> {
        match language_string.to_lowercase().as_str() {
            "english" | "en" => Ok(Languages::EN),
            "deutsch" | "german" | "de" => Ok(Languages::DE),
            _ => Err(format!("{language_string} is not a supported language.")),
        }
    }
}
