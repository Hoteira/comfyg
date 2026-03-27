use comfyg::Config;
use comfyg::types::Types;
use std::{fs::File, io::Read};

fn main() {
    let mut f = File::open("test.txt").unwrap();
    let mut data = vec![];
    f.read_to_end(&mut data).unwrap();

    let typedefs = std::collections::HashMap::from([
        ("nval", Types::Bool),
        ("v", Types::Number),
        ("bg_color", Types::Color),
    ]);

    let mut c = Config::new();
    c.load_file(&data);
    c.load_types(&typedefs);

    let val = c.get("bg_color").unwrap();
    println!("{:?}", *val);
}
