mod io;
mod templates;

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
}

fn main() {
    let args = Args::parse();
    generate_pdf(args.resume_data_path, args.target_path)
}

pub fn generate_pdf(resume_data_path: PathBuf, target_path: PathBuf) {
    let mut resume_data = load_json_resume(&resume_data_path).unwrap();

    if let Some(ref mut basics) = resume_data.basics {
        basics.image = resolve_image_path(&resume_data_path, &basics.image);
    }

    let template = Coruscant::try_from(resume_data).unwrap();

    let html_resume = template.build();
    save_to_pdf(html_resume, &target_path).expect("Failed to write to PDF.");
}
