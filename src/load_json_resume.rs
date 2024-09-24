use std::{error::Error, fs, path::PathBuf};

pub fn load_json_resume(path: &PathBuf) -> Result<json_resume::Resume, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    let resume_data = match path.extension().unwrap().to_str() {
        Some("json") => serde_json::from_str(&contents)?,
        Some("yaml") | Some("yml") => serde_yaml::from_str(&contents)?,
        _ => Err(format!(
            "Unsupported file extension: {:?}",
            path.extension()
        ))?,
    };

    Ok(resume_data)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn json() {
        let contents = "
        {
            \"basics\": {
                \"name\": \"Kirk\"
            }
        }
        ";

        let tmp_dir = TempDir::new().unwrap();
        let file_path = tmp_dir.path().join("resume.json");
        fs::write(&file_path, contents).unwrap();

        let resume_data = load_json_resume(&file_path).unwrap();
        assert_eq!(resume_data.basics.unwrap().name, Some("Kirk".to_string()));
    }

    #[test]
    fn yaml() {
        let contents = "
        basics:
            name: Kirk
        ";

        let tmp_dir = TempDir::new().unwrap();
        let file_path = tmp_dir.path().join("resume.yaml");
        fs::write(&file_path, contents).unwrap();

        let resume_data = load_json_resume(&file_path).unwrap();
        assert_eq!(resume_data.basics.unwrap().name, Some("Kirk".to_string()));
    }

    #[test]
    fn unsupported_type() {
        let contents = "";

        let tmp_dir = TempDir::new().unwrap();
        let file_path = tmp_dir.path().join("resume.UNSUPPORTED");
        fs::write(&file_path, contents).unwrap();

        let resume_data = load_json_resume(&file_path);
        assert!(resume_data.is_err());
    }
}
