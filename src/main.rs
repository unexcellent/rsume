mod generate_pdf;
mod load_json_resume;
mod resolve_image_path;
mod save_to_pdf;
mod templates;

use std::path::PathBuf;

use clap::Parser;
use generate_pdf::generate_pdf;

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
