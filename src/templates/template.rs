use json_resume::Resume;

use crate::Languages;

pub trait Template {
    /// Construct the template.
    fn new(json_resume_data: Resume, language: &Languages) -> Result<Self, String>
    where
        Self: std::marker::Sized;

    /// Return the resume as standalone HTML containing all CSS.
    fn build(&self) -> String;
}
