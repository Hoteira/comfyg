use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::types::Parse;

mod types;

fn main() {
    let f = File::open("test.txt").unwrap();
    let reader = BufReader::new(f);
    let mut map = std::collections::HashMap::new();
    let typedefs: std::collections::HashMap<&str, types::Types> =
        std::collections::HashMap::from([
            ("nval", types::Types::Bool),
            ("v", types::Types::Number),
            ("bg_color", types::Types::Color),
        ]);

    for lines in reader.lines() {
        let line = lines.unwrap();
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((attr_name, raw_value)) = line.split_once('=') {
            let name = attr_name.trim();
            let value_with_potential_comment = raw_value.trim();

            let attr_value = if value_with_potential_comment.starts_with('#') {
                value_with_potential_comment
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
            } else {
                value_with_potential_comment
                    .split('#')
                    .next()
                    .unwrap_or("")
                    .trim()
            };

            let attr_value_type = typedefs.get(name).unwrap_or(&types::Types::String);

            match attr_value_type.parse(attr_value) {
                Ok(parsed_value) => {
                    println!("{} : {} => {:#?}", name, attr_value, parsed_value);
                    map.insert(name.to_string(), parsed_value);
                }

                Err(e) => {
                    eprintln!("Error parsing '{}': {:?}", name, e);
                }
            }
        }
    }
}

pub struct Config<'a> {
    typedefs: Option<&'a std::collections::HashMap<String, types::Types>>,
    file: Option<&'a [u8]>,
    cached_map: Option<std::collections::HashMap<String, types::ReturnTypes>>,
}

impl<'a> Config<'a> {
    pub fn new() -> Self {
        Self {
            typedefs: None,
            file: None,
            cached_map: None,
        }
    }

    pub fn load_types(&mut self, typedefs: &'a std::collections::HashMap<String, types::Types>) {
        self.typedefs = Some(typedefs);
    }

    pub fn load_file(&mut self, file: &'a [u8]) {
        self.file = Some(file);
    }

    pub fn parse(&self) -> &std::collections::HashMap<String, types::ReturnTypes> {
        if self.typedefs.is_none() || self.file.is_none() {
            return &std::collections::HashMap::new();
        }

        for line_bytes in self.file.unwrap().split(|&b| b == b'\n') {
            let mut map = std::collections::HashMap::new();
            let typedefs: std::collections::HashMap<&str, types::Types> =
                std::collections::HashMap::from([
                    ("nval", types::Types::Bool),
                    ("v", types::Types::Number),
                    ("bg_color", types::Types::Color),
                ]);

            for line_bytes in self.file.unwrap().split(|&b| b == b'\n') {
                let Ok(line) = core::str::from_utf8(line_bytes) else {
                    continue;
                };

                let line = line.trim();

                if line.is_empty() || line.starts_with('#') {
                    continue;
                }

                if let Some((attr_name, raw_value)) = line.split_once('=') {
                    let name = attr_name.trim();
                    let value_with_potential_comment = raw_value.trim();

                    let attr_value = if value_with_potential_comment.starts_with('#') {
                        value_with_potential_comment
                            .split_whitespace()
                            .next()
                            .unwrap_or("")
                    } else {
                        value_with_potential_comment
                            .split('#')
                            .next()
                            .unwrap_or("")
                            .trim()
                    };

                    let attr_value_type = typedefs.get(name).unwrap_or(&types::Types::String);

                    match attr_value_type.parse(attr_value) {
                        Ok(parsed_value) => {
                            println!("{} : {} => {:#?}", name, attr_value, parsed_value);
                            map.insert(name.to_string(), parsed_value);
                        }

                        Err(e) => {
                            println!("Error parsing '{}': {:?}", name, e);
                        }
                    }
                }
            }
        }

        self.cached_map = Some(map);

        &map
    }
}
