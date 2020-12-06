//! Simple function based template engine.
//!
//! Renders template strings with functions embedded, e.g. `"<!$ echo Hello>, World" // -> "Hello, World!"`
//!
//! # Examples
//!
//! ## Echo
//! ```
//! struct Echo();
//! impl funcy::PlaceholderFunction for Echo {
//!     fn placeholder_fn_handler<'a>(&mut self, _name: &'a str, arg: &'a str) -> Result<String, String> {
//!         Ok(arg.to_string())
//!     }
//! }
//!
//! let mut tr = funcy::TemplateRenderer::with_template("<!$ echo Hello>, World!");
//! tr.set_placeholder_fn("echo", Box::new(Echo()));
//! assert_eq!(tr.render().unwrap(), "Hello, World!");
//! ```
//!
//! ## Counter
//! ```
//! struct Counter(usize);
//! impl funcy::PlaceholderFunction for Counter {
//!     fn placeholder_fn_handler<'a>(&mut self, _name: &'a str, _arg: &'a str) -> Result<String, String> {
//!         self.0 += 1;
//!         Ok(self.0.to_string())
//!     }
//! }
//!
//! let mut tr = funcy::TemplateRenderer::with_template("<!$ counter> <!$ counter> <!$ counter>");
//! let counter = Counter(0);
//! tr.set_placeholder_fn("counter", Box::new(counter));
//! assert_eq!(tr.render().unwrap(), "1 2 3");
//! ```

#![warn(missing_docs)]

mod template_renderer;
mod tests;
pub use template_renderer::{TemplateRenderer, PlaceholderFunction, RenderError};