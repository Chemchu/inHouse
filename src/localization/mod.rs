use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize)]
pub struct Translator<'a> {
    pub locale: Locale<'a>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Locale<'a> {
    pub language_key: &'a str,
    pub locale_path: &'a str,
}

impl<'a> Locale<'a> {
    pub fn new(language_key: Option<&'a str>, locale_path: Option<&'a str>) -> Self {
        Locale {
            language_key: match language_key {
                Some(key) => key,
                None => "es_ES",
            },
            locale_path: match locale_path {
                Some(path) => path,
                None => "src/locales/",
            },
        }
    }
}

trait Translate<'a> {
    fn default() -> Self;
    fn new(locale: Locale<'a>) -> Self;
    fn translate(&self, key: &'a str) -> &str;
}

impl<'a> Translate<'a> for Translator<'a> {
    fn default() -> Self {
        Translator {
            locale: Locale::new(None, None),
        }
    }

    fn new(locale: Locale<'a>) -> Self {
        Translator { locale }
    }

    /// Returns the translated string for the given key.
    /// TODO: Implement this function when you have a localization system.
    /// Step 1: Read the files from the locale_path.
    /// Step 2: For each file, create a HashMap with the key and the value.
    /// Step 3: Return the value for the given key.
    fn translate(&self, key: &'a str) -> &str {
        key
    }
}
