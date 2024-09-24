mod load_json_resume;
mod resolve_image_path;
mod save_to_pdf;
mod templates;

use std::path::PathBuf;

use clap::Parser;
use load_json_resume::load_json_resume;
use resolve_image_path::resolve_image_path;
use save_to_pdf::save_to_pdf;
use templates::Coruscant;

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

    if let Some(ref mut basics) = resume_data.basics {
        basics.image = resolve_image_path(&args.resume_data_path, &basics.image);
    }

    let template = Coruscant::new(resume_data);

    let html_resume = template.build();
    save_to_pdf(html_resume, &args.target_path);
}
