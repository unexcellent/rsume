use std::path::PathBuf;

use crate::load_json_resume::load_json_resume;
use crate::resolve_image_path::resolve_image_path;
use crate::save_to_pdf::save_to_pdf;
use crate::templates::Coruscant;

pub fn generate_pdf(resume_data_path: PathBuf, target_path: PathBuf) {
    let mut resume_data = load_json_resume(&resume_data_path).unwrap();

    if let Some(ref mut basics) = resume_data.basics {
        basics.image = resolve_image_path(&resume_data_path, &basics.image);
    }

    let template = Coruscant::try_from(resume_data).unwrap();

    let html_resume = template.build();
    save_to_pdf(html_resume, &target_path).expect("Failed to write to PDF.");
}
