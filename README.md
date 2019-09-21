# Render

> A simple template engine with the type-safety of Rust and the ergonomics of JSX

The `Renderable` trait contains a simple function that returns `String`. This is very handy for type-safe HTML templates, but can also work for writing tree-like terminal coloring mechanism like ReasonML's [Pastel](https://reason-native.com/docs/pastel/).

## Why this is different from `typed-html`?

`typed-html` is a wonderful library. Unfortunately, it focused its power in strictness of the HTML spec itself, and doesn't allow arbitrary compositions of custom elements.

`render` takes a different approach. For now, HTML is not typed at all. It can get any key and get any string value. The main focus is custom components, so you can create a composable and declarative template with no runtime errors.

## Usage

```rust
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
```
