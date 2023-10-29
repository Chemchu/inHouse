use std::{collections::HashMap, fs::File, io::Read};

use axum::http::HeaderMap;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TranslationKey {
    pub key: (String, String),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TranslationValue {
    pub value: String,
}

#[derive(Clone, Serialize)]
pub struct Translator {
    pub locale: String,
    pub locale_path: &'static str,
    pub languages: Vec<String>,
    pub translations: HashMap<TranslationKey, TranslationValue>,
}

static DEFAULT_LOCALE: &'static str = "en_US";

impl Translator {
    pub fn new(locale: String, locale_path: &'static str) -> Self {
        let translations = init_translator(locale_path);
        Translator {
            locale,
            locale_path,
            languages: translations.keys().map(|k| k.key.1.to_string()).collect(),
            translations,
        }
    }

    fn translate_with_locale(&self, key: &'static str, locale: &'static str) -> String {
        let translate_key = TranslationKey {
            key: (key.to_string(), locale.to_string()),
        };

        self.translations
            .get(&translate_key)
            .expect(format!("Translation not found for key: {}", key).as_str())
            .value
            .clone()
    }

    pub fn translate(&self, key: &'static str) -> String {
        let translate_key = TranslationKey {
            key: (key.to_string(), self.locale.to_string()),
        };

        self.translations
            .get(&translate_key)
            .unwrap_or(&TranslationValue {
                value: self.translate_with_locale(key, DEFAULT_LOCALE),
            })
            .value
            .clone()
    }
}

fn init_translator<'a>(locale_path: &'a str) -> HashMap<TranslationKey, TranslationValue> {
    // Create a HashMap to store your key-value pairs as (String, String) tuples
    let mut key_value_map: HashMap<TranslationKey, TranslationValue> = HashMap::new();

    for entry in WalkDir::new(locale_path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() && entry.file_name().to_str().unwrap().ends_with(".yaml") {
            // If you found the YAML file, try to open and parse it
            let file_path = entry.path();

            // Open the file and read its contents
            let mut file = match File::open(&file_path) {
                Ok(file) => file,
                Err(_) => {
                    eprintln!("Failed to open file: {}", file_path.display());
                    continue;
                }
            };

            let mut yml = String::new();
            if let Err(_) = file.read_to_string(&mut yml) {
                eprintln!("Failed to read file: {}", file_path.display());
                continue;
            }

            // Parse the YAML content into a temporary HashMap
            let mut temp_map: HashMap<String, HashMap<String, String>> =
                match serde_yaml::from_str(&yml) {
                    Ok(map) => map,
                    Err(err) => {
                        eprintln!("Failed to parse YAML: {}", err);
                        continue;
                    }
                };

            // Clone and insert the values from the temporary map into the final key_value_map
            for (key, value_map) in temp_map.drain() {
                for (lang, value) in value_map {
                    key_value_map.insert(
                        TranslationKey {
                            key: (key.clone(), lang.clone()),
                        },
                        TranslationValue { value: value },
                    );
                }
            }
        }
    }

    key_value_map
}

pub fn get_locale_from_headers(headers: &HeaderMap, defined_locales: Vec<String>) -> String {
    if let Some(accept_language) = headers.get("accept-language") {
        let accept_language = accept_language.to_str().unwrap();

        let mut languages: Vec<&str> = accept_language.split(',').collect();

        languages.sort_by(|a, b| {
            let a = a.split(';').collect::<Vec<&str>>()[0];
            let b = b.split(';').collect::<Vec<&str>>()[0];

            let a = a.split('-').collect::<Vec<&str>>();
            let b = b.split('-').collect::<Vec<&str>>();

            if a.len() == 2 && b.len() == 2 {
                if a[0] == b[0] {
                    return a[1].cmp(&b[1]);
                }
            }

            a.len().cmp(&b.len())
        });

        for language in languages {
            let language = language.split(';').collect::<Vec<&str>>()[0].to_string();
            if defined_locales.contains(&language) {
                return language.to_string();
            }

            if let Some(element) = defined_locales
                .iter()
                .find(|string| string.starts_with(&language))
            {
                return element.to_string();
            }
        }
    }

    DEFAULT_LOCALE.to_string()
}
