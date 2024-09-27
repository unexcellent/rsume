use serde::Serialize;

use super::utils::get_mandatory_field;

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize)]
pub struct Language {
    pub language: String,
    pub fluency: String,
}
impl Language {
    pub fn try_from(language: json_resume::Language) -> Result<Self, String> {
        Ok(Self {
            language: get_mandatory_field(language.language, "language.language")?,
            fluency: get_mandatory_field(language.fluency, "language.fluency")?,
        })
    }

    pub fn percentage(&self) -> Result<u8, ()> {
        if self.fluency.ends_with("%") {
            match remove_last_char(&self.fluency).parse() {
                Ok(p) => return Ok(p),
                Err(_) => return Err(()),
            }
        }

        match self.fluency.as_str() {
            "A1" => Ok(17),
            "A2" => Ok(33),
            "B1" => Ok(50),
            "B2" => Ok(67),
            "C1" => Ok(83),
            "C2" => Ok(100),
            _ => Err(()),
        }
    }
}

fn remove_last_char(string: &str) -> &str {
    match string.char_indices().next_back() {
        Some((i, _)) => &string[..i],
        None => string,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percentage_from_number() {
        let language = Language {
            language: "".to_string(),
            fluency: "70%".to_string(),
        };

        let actual = language.percentage();
        assert_eq!(actual, Ok(70));
    }

    #[test]
    fn percentage_from_level() {
        let language = Language {
            language: "".to_string(),
            fluency: "B2".to_string(),
        };

        let actual = language.percentage();
        assert_eq!(actual, Ok(67));
    }

    #[test]
    fn percentage_unknown() {
        let language = Language {
            language: "".to_string(),
            fluency: "UNKNOWN".to_string(),
        };

        let actual = language.percentage();
        assert_eq!(actual, Err(()));
    }
}
