use std::path::Path;

/// Image paths can either be given as absolute or relative paths from the JSONResume file. This function resolves relative paths.
pub fn resolve_image_path(resume_data_path: &Path, image_path: &Option<String>) -> Option<String> {
    image_path.as_ref().map(|path| {
        resume_data_path
            .parent()
            .unwrap()
            .join(path)
            .to_str()
            .unwrap()
            .to_string()
    })
}

#[cfg(test)]
pub mod tests {
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
        let resume_data_path = PathBuf::from("/path/to/some/resume.json");
        let image_path = Some("profile.png".to_string());

        let actual = resolve_image_path(&resume_data_path, &image_path);
        assert_eq!(actual, Some("/path/to/some/profile.png".to_string()))
    }

    #[test]
    fn neighboring_folder() {
        let resume_data_path = PathBuf::from("/path/to/some/resume.json");
        let image_path = Some("../images/profile.png".to_string());

        let actual = resolve_image_path(&resume_data_path, &image_path);
        assert_eq!(
            actual,
            Some("/path/to/some/../images/profile.png".to_string())
        )
    }
}
