use std::{collections::HashMap, fs::File, io::Read};

use axum::extract::FromRef;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use crate::domain::AppState;

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TranslationKey {
    pub key: (String, String),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TranslationValue {
    pub value: String,
}

#[derive(Clone, Serialize)]
pub struct Translator<'a> {
    pub default_locale: &'a str,
    pub locale_path: &'a str,
    pub translations: HashMap<TranslationKey, TranslationValue>,
}

impl<'a> Translator<'a> {
    pub fn new(default_locale: &'a str, locale_path: &'a str) -> Self {
        Translator {
            default_locale,
            locale_path,
            translations: init_translator(locale_path),
        }
    }

    pub fn translate(&self, key: &'a str, locale: Option<&'a str>) -> String {
        let translate_key = TranslationKey {
            key: (
                key.to_string(),
                match locale {
                    Some(locale) => locale.to_string(),
                    None => self.default_locale.to_string(),
                },
            ),
        };

        self.translations
            .get(&translate_key)
            .unwrap_or(&TranslationValue {
                value: key.to_string(),
            })
            .value
            .clone()
    }
}

impl<'a> FromRef<AppState<'a>> for Translator<'a> {
    fn from_ref(input: &AppState<'a>) -> Self {
        input.translator.clone()
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
