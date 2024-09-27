use std::{fs, path::Path};

/// Image paths can either be given as absolute or relative paths from the JSONResume file. This function resolves relative paths.
pub fn resolve_image_path(resume_data_path: &Path, image_path: &Option<String>) -> Option<String> {
    image_path.as_ref().map(|img_path| {
        fs::canonicalize(resume_data_path.parent().unwrap().join(img_path))
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    })
}

#[cfg(test)]
pub mod tests {
    use tempfile::TempDir;

    use super::*;
    use std::path::PathBuf;

    #[test]
    fn none() {
        let resume_data_path = PathBuf::from("/path/to/some/resume.json");
        let image_path = None;

        let actual = resolve_image_path(&resume_data_path, &image_path);
        assert_eq!(actual, None)
    }

    #[test]
    fn same_folder() {
        let tmp_dir = TempDir::new().unwrap();
        let resume_data_path = tmp_dir.path().join("resume.json");
        let image_path = Some("profile.png".to_string());

        fs::write(&resume_data_path, "").unwrap();
        fs::write(tmp_dir.path().join("profile.png"), "").unwrap();

        let actual = resolve_image_path(&resume_data_path, &image_path);
        assert!(PathBuf::from(actual.unwrap()).is_file())
    }
}
