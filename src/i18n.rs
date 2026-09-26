use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Language {
    #[default]
    Fr,
    En,
}

impl Language {
    pub fn toggle(&mut self) {
        *self = match self {
            Language::Fr => Language::En,
            Language::En => Language::Fr,
        };
    }

    pub fn code(&self) -> &'static str {
        match self {
            Language::Fr => "FR",
            Language::En => "EN",
        }
    }
}

pub fn tr<'a>(lang: Language, fr: &'a str, en: &'a str) -> &'a str {
    match lang {
        Language::Fr => fr,
        Language::En => en,
    }
}
