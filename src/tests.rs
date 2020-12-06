struct Echo();
impl crate::PlaceholderFunction for Echo {
    fn placeholder_fn_handler<'a>(&mut self, _name: &'a str, arg: &'a str) -> Result<String, String> {
        Ok(arg.to_string())
    }
}

#[test]
fn echo_function() {
    let mut tr = crate::TemplateRenderer::with_template("<!$ echo test>");
    tr.set_placeholder_fn("echo", Box::new(Echo()));
    assert_eq!(tr.render().unwrap(), "test");

    tr.set_template("<!$ echo test with spaces> and extra text");
    assert_eq!(tr.render().unwrap(), "test with spaces and extra text");
}

struct Counter(usize);
impl crate::PlaceholderFunction for Counter {
    fn placeholder_fn_handler<'a>(&mut self, _name: &'a str, _arg: &'a str) -> Result<String, String> {
        self.0 += 1;
        Ok(self.0.to_string())
    }
}

#[test]
fn counter_function() {
    let mut tr = crate::TemplateRenderer::with_template("<!$ counter> <!$ counter>");
    tr.set_placeholder_fn("counter", Box::new(Counter(0)));
    assert_eq!(tr.render().unwrap(), "1 2");
}

#[test]
fn nonexistent_function() {
    let mut tr = crate::TemplateRenderer::with_template("<!$ nonexistent>");
    assert!(match tr.render().unwrap_err() {
        crate::RenderError::UnknownFunction(_) => true,
        _ => false
    });
}

struct RetErr();
impl crate::PlaceholderFunction for RetErr {
    fn placeholder_fn_handler<'a>(&mut self, _name: &'a str, _arg: &'a str) -> Result<String, String> {
        Err("test error".to_string())
    }
}

#[test]
fn function_returning_err() {
    let mut tr = crate::TemplateRenderer::with_template("<!$ err>");
    println!("{:?}", tr);
    tr.set_placeholder_fn("err", Box::new(RetErr()));
    assert!(match tr.render().unwrap_err() {
        crate::RenderError::FunctionError("err", err_str) => err_str == "test error",
        _ => false
    });
}