mod io;
mod templates;

use std::error::Error;
use std::path::PathBuf;

use crate::io::load_json_resume::load_json_resume;
use crate::io::resolve_image_path::resolve_image_path;
use crate::io::save_to_pdf::save_to_pdf;
use crate::templates::Coruscant;
use clap::Parser;
use templates::template::Template;

/// Program for generating a resume from JSONResume data.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to the data describing your resume. It needs to comply with theJSONResume schema (see https://jsonresume.org/).
    resume_data_path: PathBuf,

    /// Location where the generated PDF should be stored.
    target_path: PathBuf,

    /// Template that should be used. Currently, the only available option is 'coruscant'. Default is 'coruscant'.
    #[arg(short, long)]
    template: Option<String>,

    /// Language of the template. Available options are 'english' and 'deutsch'. Default is english.
    #[arg(short, long)]
    language: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let language = match args.language {
        Some(language_string) => GloballySupportedLanguages::try_from(language_string)?,
        None => GloballySupportedLanguages::EN,
    };

    let template = match args.template {
        Some(template_string) => AvailableTemplates::try_from(template_string)?,
        None => AvailableTemplates::Coruscant,
    };

    generate_pdf(args.resume_data_path, args.target_path, template, language)?;

    Ok(())
}

/// Generate a resume and save it as a PDF.
pub fn generate_pdf(
    resume_data_path: PathBuf,
    target_path: PathBuf,
    template_enum: AvailableTemplates,
    language: GloballySupportedLanguages,
) -> Result<(), Box<dyn Error>> {
    let mut resume_data = load_json_resume(&resume_data_path).unwrap();

    if let Some(ref mut basics) = resume_data.basics {
        basics.image = resolve_image_path(&resume_data_path, &basics.image);
    }

    let template = match template_enum {
        AvailableTemplates::Coruscant => Coruscant::new(resume_data, &language).unwrap(),
    };

    let html_resume = template.build();
    save_to_pdf(html_resume, &target_path)?;

    Ok(())
}

/// All templates that are available.
pub enum AvailableTemplates {
    Coruscant,
}
impl AvailableTemplates {
    pub fn try_from(template_string: String) -> Result<Self, String> {
        match template_string.to_lowercase().as_str() {
            "coruscant" => Ok(AvailableTemplates::Coruscant),
            _ => Err(format!("{template_string} is not a supported template.")),
        }
    }
}

/// Language in which the resume should be generated in.
pub enum GloballySupportedLanguages {
    /// English
    EN,
    /// German
    DE,
}
impl GloballySupportedLanguages {
    pub fn try_from(language_string: String) -> Result<Self, String> {
        match language_string.to_lowercase().as_str() {
            "english" | "en" => Ok(GloballySupportedLanguages::EN),
            "deutsch" | "german" | "de" => Ok(GloballySupportedLanguages::DE),
            _ => Err(format!("{language_string} is not a supported language.")),
        }
    }
}
