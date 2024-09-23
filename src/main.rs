mod load_json_resume;
mod resolve_image_path;
mod templates;

use std::path::PathBuf;

use clap::Parser;
use load_json_resume::load_json_resume;
use resolve_image_path::resolve_image_path;

/// Program for generating a resume from JSONResume data.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the data describing your resume. It needs to comply with theJSONResume schema (see https://jsonresume.org/).
    resume_data_path: PathBuf,

    /// Location where the generated PDF should be stored.
    target_path: PathBuf,
}

fn main() {
    let args = Args::parse();

    let mut resume_data = load_json_resume(&args.resume_data_path).unwrap();
    resume_data.basics.unwrap().image = resolve_image_path(
        &args.resume_data_path,
        &resume_data.basics.clone().unwrap().image,
    )
}
