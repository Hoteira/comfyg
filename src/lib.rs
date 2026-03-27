use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::types::Parse;

mod types;

pub struct Config<'a> {
    typedefs: Option<&'a std::collections::HashMap<String, types::Types>>,
    file: Option<&'a [u8]>,
    cached_map: std::collections::HashMap<String, types::ReturnTypes>,
}

impl<'a> Config<'a> {
    pub fn new() -> Self {
        Self {
            typedefs: None,
            file: None,
            cached_map: std::collections::HashMap::new(),
        }
    }

    pub fn load_types(&mut self, typedefs: &'a std::collections::HashMap<String, types::Types>) {
        self.typedefs = Some(typedefs);
    }

    pub fn load_file(&mut self, file: &'a [u8]) {
        self.file = Some(file);
    }

    pub fn parse(&mut self) {
        let (Some(typedefs), Some(file_bytes)) = (self.typedefs, self.file) else {
            return;
        };

        self.cached_map.clear();

        for line_bytes in file_bytes.split(|&b| b == b'\n') {
            let Ok(line) = core::str::from_utf8(line_bytes) else {
                continue;
            };
            let line = line.trim();

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((attr_name, raw_value)) = line.split_once('=') {
                let name = attr_name.trim();
                let value_part = raw_value.trim();

                let attr_value = if value_part.starts_with('#') {
                    value_part.split_whitespace().next().unwrap_or("")
                } else {
                    value_part.split('#').next().unwrap_or("").trim()
                };

                let attr_value_type = typedefs.get(name).unwrap_or(&types::Types::String);

                if let Ok(parsed_value) = attr_value_type.parse(attr_value) {
                    println!("{} : {} => {:#?}", name, attr_value, parsed_value);
                    self.cached_map.insert(name.to_string(), parsed_value);
                }
            }
        }
    }

    pub fn get(&mut self, key: &str) -> Option<&types::ReturnTypes> {
        if self.cached_map.is_empty() {
            self.parse();
        }

        self.cached_map.get(key)
    }
}
