use std::{error::Error, fs, path::PathBuf};

use headless_chrome::{types::PrintToPdfOptions, Browser};
use tempfile::TempDir;
use typed_html::dom::DOMTree;

pub fn save_to_pdf(
    html_resume: DOMTree<String>,
    target_path: &PathBuf,
) -> Result<(), Box<dyn Error>> {
    let tmp_dir = TempDir::new()?;
    let tmp_storage_path = tmp_dir.path().join("resume.html");
    fs::write(&tmp_storage_path, html_resume.to_string())?;

    let browser = Browser::default()?;
    let tab = browser.new_tab()?;
    let pdf = tab
        .navigate_to(&format!("file://{}", tmp_storage_path.to_str().unwrap()))?
        .wait_until_navigated()?
        .print_to_pdf(Some(PrintToPdfOptions {
            margin_bottom: Some(0.0),
            margin_top: Some(0.0),
            margin_left: Some(0.0),
            margin_right: Some(0.0),
            ..PrintToPdfOptions::default()
        }))?;

    fs::write(target_path, pdf)?;

    Ok(())
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use typed_html::html;

    #[test]
    fn file_is_written() {
        let html_resume: DOMTree<String> = html!(
            <html>
                <head>
                    <title>"Hello world"</title>
                </head>
                <body>
                    <h1>"Hello world"</h1>
                </body>
            </html>
        );

        let tmp_dir = TempDir::new().unwrap();
        let target_path = tmp_dir.path().join("resume.pdf");

        let result = save_to_pdf(html_resume, &target_path);
        assert!(result.is_ok());
        assert!(target_path.is_file());
    }
}
