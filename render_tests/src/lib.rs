#![feature(proc_macro_hygiene)]

use render::html::HTML5Doctype;
use render::{html, rsx, Fragment, Renderable};

#[derive(Debug)]
struct Hello<'a, T: Renderable> {
    world: &'a str,
    yes: i32,
    children: T,
}

impl<'a, T: Renderable> Renderable for Hello<'a, T> {
    fn render(self) -> String {
        html! {
            <b class={"some_bem_class"}>
                {format!("{}", self.world)}
                <br />
                {format!("A number: {}", self.yes)}
                {self.children}
            </b>
        }
    }
}

pub fn it_works() -> String {
    let world = "hello";
    let other_value = rsx! {
        <em>{format!("hello world?")}</em>
    };
    let value = html! {
        <Fragment>
            <HTML5Doctype />
            <Hello world yes={1 + 1}>
                <div>{format!("HEY!")}</div>
                {other_value}
            </Hello>
        </Fragment>
    };
    value
}

#[test]
pub fn verify_works() {
    println!("{}", it_works());
}

pub mod readme_code {
    // A simple HTML 5 doctype declaration
    use render::html::HTML5Doctype;
    use render::{
        // A macro to compose components in JSX fashion
        html,
        // A component that just render its children
        Fragment,
        // A trait for custom components
        Renderable,
    };

    // This can be any layout we want
    #[derive(Debug)]
    struct Page<'a, T: Renderable> {
        title: &'a str,
        children: T,
    }

    // Implementing `Renderable` gives the ability to compose
    // components
    impl<'a, T: Renderable> Renderable for Page<'a, T> {
        fn render(self) -> String {
            html! {
              <Fragment>
                <HTML5Doctype />
                <html>
                  <head><title>{self.title}</title></head>
                  <body>
                    {self.children}
                  </body>
                </html>
              </Fragment>
            }
        }
    }

    // This can be a route in Rocket, the web framework,
    // for instance.
    pub fn some_page(user_name: &str) -> String {
        html! {
          <Page title={"Home"}>
            {format!("Welcome, {}", user_name)}
          </Page>
        }
    }
}

#[test]
fn test_readme_stuff() {
    use pretty_assertions::assert_eq;
    let result = readme_code::some_page("Gal");
    let expected = concat!(
        "<!DOCTYPE html>",
        "<html>",
        "<head><title>Home</title></head>",
        "<body>",
        "Welcome, Gal",
        "</body>",
        "</html>"
    );

    assert_eq!(result, expected);
}
