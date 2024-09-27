use minijinja::Environment;
use serde::Serialize;

pub fn render_template<S: Serialize>(template: &str, context: S) -> Result<String, String> {
    let mut environment = Environment::new();
    environment.add_template("", template).unwrap();

    let template = environment.get_template("").unwrap();

    match template.render(context) {
        Ok(t) => Ok(t),
        Err(_) => Err("Could not render template".to_string()),
    }
}
