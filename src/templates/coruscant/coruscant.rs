use json_resume::Resume;
use typed_html::dom::DOMTree;

pub struct Coruscant;

impl Coruscant {
    pub fn new(json_resume_data: Resume) -> Self {
        unimplemented!()
    }

    /// Build the resume as printable HTML.
    pub fn build(&self) -> DOMTree<String> {
        unimplemented!()
    }
}
