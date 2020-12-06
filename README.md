# Funcy [![Crates.io Status](https://img.shields.io/crates/v/funcy.svg)](https://crates.io/crates/funcy) [![Documentation](https://docs.rs/funcy/badge.svg)](https://docs.rs/funcy/)
Funcy is a simple function based template engine.

## Examples

### Echo
```rust
struct Echo();
impl funcy::PlaceholderFunction for Echo {
    fn placeholder_fn_handler<'a>(&mut self, _name: &'a str, arg: &'a str) -> Result<String, String> {
        Ok(arg.to_string())
    }
}

let mut tr = funcy::TemplateRenderer::with_template("<!$ echo Hello>, World!");
tr.set_placeholder_fn("echo", Box::new(Echo()));
assert_eq!(tr.render().unwrap(), "Hello, World!");
```

### Counter
```rust
struct Counter(usize);
impl funcy::PlaceholderFunction for Counter {
    fn placeholder_fn_handler<'a>(&mut self, _name: &'a str, _arg: &'a str) -> Result<String, String> {
        self.0 += 1;
        Ok(self.0.to_string())
    }
}

let mut tr = funcy::TemplateRenderer::with_template("<!$ counter> <!$ counter> <!$ counter>")
let counter = Counter(0);
tr.set_placeholder_fn("counter", Box::new(counter));
assert_eq!(tr.render().unwrap(), "1 2 3");
```

## License
Funcy is distributed under the Apache-2.0 license. See the [LICENSE](LICENSE) file.