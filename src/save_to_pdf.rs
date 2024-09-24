use std::path::PathBuf;

use typed_html::dom::DOMTree;

pub fn save_to_pdf(html_resume: DOMTree<String>, target_path: &PathBuf) {
    unimplemented!()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use tempfile::TempDir;
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

        save_to_pdf(html_resume, &target_path);
        assert!(target_path.is_file());
    }
}
