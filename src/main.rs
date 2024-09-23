use std::path::PathBuf;

use clap::Parser;

mod load_json_resume;

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
    let _args = Args::parse();
}
