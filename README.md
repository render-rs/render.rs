# render

> 🔏 A safe and simple template engine with the ergonomics of JSX

`render` itself is a combination of traits, structs and macros that together unify and
boost the experience of composing tree-shaped data structures. This works best with HTML and
XML rendering, but can work with other usages as well, like ReasonML's [`Pastel`](https://reason-native.com/docs/pastel/) library for terminal colors.

## How?

A renderable component is a struct that implements the `Renderable` trait. There
are multiple macros that provide a better experience implementing Renderable:

* `html!` for the JSX ergonomics
* `#[component]` for the syntactic-sugar of function components

## Why this is different from `typed-html`?

`typed-html` is a wonderful library. Unfortunately, it focused its power in strictness of the HTML spec itself, and doesn't allow arbitrary compositions of custom elements.

`render` takes a different approach. For now, HTML is not typed at all. It can get any key and get any string value. The main focus is custom components, so you can create a composable and declarative template with no runtime errors.

## Usage

```rust
#![feature(proc_macro_hygiene)]

// A simple HTML 5 doctype declaration
use render::html::HTML5Doctype;
use render::{
    // A macro to create components
    component,
    // A macro to compose components in JSX fashion
    rsx,
    // A macro to render components in JSX fashion
    html,
    // A trait for custom components
    Render,
};

// This can be any layout we want
#[component]
fn Page<'a, Children: Render>(title: &'a str, children: Children) {
   rsx! {
     <>
       <HTML5Doctype />
       <html>
         <head><title>{title}</title></head>
         <body>
           {children}
         </body>
       </html>
     </>
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

License: MIT
