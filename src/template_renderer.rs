/*
Copyright 2020 camc

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use std::collections::HashMap;


/// Renders Funcy template strings.
/// 
/// # Example
/// 
/// ```
/// struct Echo();
/// impl funcy::PlaceholderFunction for Echo {
///     fn placeholder_fn_handler<'a>(&mut self, _name: &'a str, arg: &'a str) -> Result<String, String> {
///         Ok(arg.to_string())
///     }
/// }
///
/// let mut tr = funcy::TemplateRenderer::with_template("<!$ echo Hello>, World!");
/// tr.set_placeholder_fn("echo", Box::new(Echo()));
/// assert_eq!(tr.render().unwrap(), "Hello, World!");
/// ```
#[derive(Default)]
pub struct TemplateRenderer<'a> {
    template_str: &'a str,
    placeholders: Vec<PlaceholderExpr<'a>>,
    placeholder_functions: HashMap<&'a str, Box<dyn PlaceholderFunction>>
}

impl<'a> TemplateRenderer<'a> {
    /// Creates a [`TemplateRenderer`] with an empty template
    pub fn new() -> Self {
        Self {
            template_str: "",
            placeholders: Vec::new(),
            placeholder_functions: HashMap::new()
        }
    }

    /// Creates a [`TemplateRenderer`] with the specified template
    pub fn with_template(inp_str: &'a str) -> Self {
        Self {
            template_str: inp_str,
            placeholders: parse_placeholders(inp_str),
            placeholder_functions: HashMap::new()
        }
    }

    /// Sets the renderer's template string
    pub fn set_template(&mut self, inp_str: &'a str) {
        self.template_str = inp_str;
        self.placeholders = parse_placeholders(inp_str);
    }

    /// Adds/replaces the specified placeholder function
    pub fn set_placeholder_fn(&mut self, name: &'a str, thefn: Box<dyn PlaceholderFunction>) {
        self.placeholder_functions.insert(name, thefn);
    }

    /// Appends placeholder functions from a HashMap to the current placeholder functions
    pub fn append_placeholders(&mut self, map: HashMap<&'a str, Box<dyn PlaceholderFunction>>) {
        self.placeholder_functions.extend(map);
    }

    /// Overwrites current placeholder functions with the HashMap
    pub fn set_placeholders(&mut self, map: HashMap<&'a str, Box<dyn PlaceholderFunction>>) {
        self.placeholder_functions = map;
    }

    /// Renders the template into a [`String`]
    pub fn render(&mut self) -> Result<String, RenderError> {
        let mut out_str = String::new();
        let mut last_end = 0;

        for placeholder in &self.placeholders {
            out_str.push_str(&self.template_str[last_end..placeholder.start_idx]);
            let func: &str;
            let arg: &str;
            if placeholder.content.contains(" ") {
                let fa = split_once(placeholder.content, ' ').unwrap();
                func = fa.0;
                arg = fa.1;
            } else {
                func = placeholder.content;
                arg = "";
            }

            if let Some(placeholderfn) = self.placeholder_functions.get_mut(func) {
                match placeholderfn.placeholder_fn_handler(func, arg) {
                    Ok(output) => out_str.push_str(&output),
                    Err(err) => return Err(RenderError::FunctionError(func, err)),
                }
            } else {
                return Err(RenderError::UnknownFunction(*placeholder));
            }

            last_end = placeholder.end_idx;
        }

        out_str.push_str(&self.template_str[last_end..self.template_str.len()]);
        Ok(out_str)
    }
}

impl<'a> std::fmt::Debug for TemplateRenderer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "TemplateRenderer{{ template_str: {:?}, placeholders: {:?}, placeholder_functions: {:?} }}", self.template_str, self.placeholders, self.placeholder_functions.keys())
    }
}

/// Error returned by [`TemplateRenderer::render`].
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RenderError<'a> {
    /// Returned when a placeholder referenced a function that was not found in the added functions
    UnknownFunction(PlaceholderExpr<'a>),
    /// Returned when a placeholder function returns an error.
    /// The first item is the name of the function, the second is the error string returned.
    FunctionError(&'a str, String)
}

impl std::fmt::Display for RenderError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            RenderError::UnknownFunction(placeholder) => write!(f, "Unknown function at char {} in placeholder content: '{}'", placeholder.start_idx, placeholder.content),
            RenderError::FunctionError(func, err) => write!(f, "Error in placeholder function {}: '{}'", func, err),
        }
    }
}

impl std::error::Error for RenderError<'_> {}


/// Trait used to define functions that can be called from placeholders.
/// 
/// # Example
/// 
/// ```
/// struct Echo();
/// impl funcy::PlaceholderFunction for Echo {
///     fn placeholder_fn_handler<'a>(&mut self, _name: &'a str, arg: &'a str) -> Result<String, String> {
///         Ok(arg.to_string())
///     }
/// }
/// ```
pub trait PlaceholderFunction {
    /// Called when a placeholder references the function.
    /// The arg may be empty. Errors returned will be propagated and returned from the [`TemplateRenderer::render`] function.
    /// 
    /// The name argument includes the name of the placeholder function being called. 
    /// This can be used to have one struct handle multiple placeholder functions.
    fn placeholder_fn_handler<'a>(&mut self, name: &'a str, arg: &'a str) -> Result<String, String>;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct PlaceholderExpr<'a> {
    start_idx: usize,
    end_idx: usize,
    content: &'a str
}

const PLACEHOLDER_PARTS: [char; 5] = ['<', '!', '$', ' ', '>'];
fn parse_placeholders(inp_str: &str) -> Vec<PlaceholderExpr> {
    let mut placeholders = Vec::new();
    
    let mut tmp_part = 0;
    let mut tmp_tag_start = 0;
    let mut tmp_is_in_tag = false;

    for (i, c) in inp_str.chars().enumerate() {
        if PLACEHOLDER_PARTS[tmp_part] == c {
            if tmp_is_in_tag {
                tmp_part += 1;
            } else {
                tmp_is_in_tag = true;
                tmp_tag_start = i;
                tmp_part += 1;
            }
        } else if tmp_part != 4 {
            tmp_part = 0;
            tmp_is_in_tag = false;
        }

        if tmp_part == 5 {
            placeholders.push(PlaceholderExpr {
                start_idx: tmp_tag_start,
                end_idx: i+1,
                content: &inp_str[(tmp_tag_start+4)..i]
            });

            tmp_part = 0;
            tmp_is_in_tag = false;
        }

    }

    placeholders
}

/* waiting for https://github.com/rust-lang/rust/issues/74773 */
fn split_once<'a>(inp: &'a str, delim: char) -> Option<(&'a str, &'a str)> {
    let splitted: Vec<&str> = inp.splitn(2, delim).collect();
    if splitted.len() == 2 {
        Some((splitted[0], splitted[1]))
    } else {
        None
    }
}

#[test]
fn placeholder_parsing() {
    assert_eq!(parse_placeholders("<!$ name arg>"), [PlaceholderExpr { start_idx: 0, end_idx: 13, content: "name arg" }]);
    assert_eq!(parse_placeholders("<!$ name1 arg1> <!$ name2 arg2>")
        , [PlaceholderExpr { start_idx: 0, end_idx: 15, content: "name1 arg1" }
        , PlaceholderExpr { start_idx: 16, end_idx: 31, content: "name2 arg2" }]);
    assert_eq!(parse_placeholders("some text <!$ name1 arg1> other text <!$ name2 arg2> even more text")
        , [PlaceholderExpr { start_idx: 10, end_idx: 25, content: "name1 arg1" }
        , PlaceholderExpr { start_idx: 37, end_idx: 52, content: "name2 arg2" }]);
}