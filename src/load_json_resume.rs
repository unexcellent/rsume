use std::{error::Error, fs, path::PathBuf};

pub fn load_json_resume(path: PathBuf) -> Result<json_resume::Resume, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    let resume_data = serde_json::from_str(&contents)?;

    Ok(resume_data)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn load_json() {
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

        let resume_data = load_json_resume(file_path).unwrap();
        assert_eq!(resume_data.basics.unwrap().name, Some("Kirk".to_string()));
    }
}
