# Comfyg

A minimal, `no_std`-compatible configuration file parser for custom config formats inspired by Hyprland-style syntax. Built for use in OS projects, embedded environments, and anywhere you want a lightweight, typed config parser without pulling in heavy dependencies.

---

## Format

Config files follow a simple `key = value` syntax. Comments start with `#` and can appear on their own line or inline. Lines not containing 
```=``` signs are ignored.

```
# This is a comment
nval = true
v = 42
bg_color = #FF5733FF  # inline comment
```

Values are typed — you define what type each key maps to, and the parser handles validation and conversion.

---

## Supported Types

**Bool**

Accepts: `true`, `True`, `TRUE`, `t`, `T`, `1`, `false`, `False`, `FALSE`, `f`, `F`, `0`

**Number**

Accepts decimal floats and hex integers prefixed with `0x` or `0X`.

```
v = 3.14
v = 0xFF
```

**Color**

Accepts multiple formats:

```
bg = #RRGGBB
bg = #RRGGBBAA
bg = RRGGBB
bg = RRGGBBAA
bg = 0xRRGGBBAA
bg = rgb(255, 87, 51)
bg = rgba(255, 87, 51, 255)
```

Colors are parsed into `[u8; 4]` (RGBA).

**String**

Any value not matched by another type is treated as a raw string.

---

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
comfyg = "0.1.0"
```

For `no_std` environments, disable default features:

```toml
[dependencies]
comfyg = { version = "0.1.0", default-features = false }
```

Note: `no_std` mode requires a global allocator to be configured in your crate.

### Basic example

```rust
use comfyg::Config;
use comfyg::types::Types;
use std::{fs::File, io::Read};

fn main() {
    let mut f = File::open("config.txt").unwrap();
    let mut data = vec![];
    f.read_to_end(&mut data).unwrap();

    let typedefs = std::collections::HashMap::from([
        ("nval",     Types::Bool),
        ("v",        Types::Number),
        ("bg_color", Types::Color),
        ("title",    Types::String),
    ]);

    let mut c = Config::new();
    c.load_file(&data);
    c.load_types(&typedefs);

    let val = c.get("bg_color").unwrap();
    println!("{:?}", *val);
}
```

Parsing is lazy — it happens on the first call to `get()`. Subsequent calls hit the cache. Calling `load_file` or `load_types` invalidates the cache and forces a re-parse on the next `get()`.

---

## License

MIT
