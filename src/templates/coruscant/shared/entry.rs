#[allow(dead_code)]
pub struct Entry {
    pub start_date: String,
    pub end_date: String,
    pub title: String,
    pub body: String,
}
impl Entry {
    pub fn build(&self) -> String {
        String::new()
    }
}
