# render

> 🔏 A safe and simple template engine with the ergonomics of JSX

`render` itself is a combination of traits, structs and macros that together unify and
boost the experience of composing tree-shaped data structures. This works best with HTML and
XML rendering, but can work with other usages as well, like ReasonML's [`Pastel`](https://reason-native.com/docs/pastel/) library for terminal colors.

## How?

A renderable component is a struct that implements the `Render` trait. There
are multiple macros that provide a better experience implementing Renderable:

* `#[component]` for defining components using a function
* `rsx!` for composing elements with JSX ergonomics
* `html!` for composing elements and render them to a string

## Why is this different from...

### `handlebars`?

Handlebars is an awesome spec that lets us devs define templates and work
seemlessly between languages and frameworks. Unfortunately, it does not guarantee any of Rust's
type-safety, due to its spec. This forces you to write tests for validating types for your views, like you would in a dynamically typed language. These tests weren't necessary in a type-safe language like Rust — but Handlebars is JSON-oriented, which doesn't comply Rust's type system.

`render` provides the same level of type-safety Rust provides, with no compromises of
ergonomics or speed.

### `typed-html`?

`typed-html` is a wonderful library. Unfortunately, it focused its power in strictness of the HTML spec itself, and doesn't allow arbitrary compositions of custom elements.

`render` takes a different approach. For now, HTML is not typed at all. It can get any key and get any string value. The main focus is custom components, so you can create a composable and declarative template with no runtime errors.

## Usage

> Note: `render` needs the `nightly` Rust compiler, for now, so it will have hygienic macros.

This means you will need to add the following feature flag in the root of your `lib.rs`/`main.rs`:

```rust
#![feature(proc_macro_hygiene)]
```

### Simple HTML rendering

In order to render a simple HTML fragment into a `String`, use the `rsx!` macro to generate a
component tree, and call `render` on it:

```rust
#![feature(proc_macro_hygiene)]

use render::{rsx, Render};

let tree = rsx! {
  <div>
    <h1>{"Hello!"}</h1>
    <p>{"Hello world!"}</p>
  </div>
};

assert_eq!(tree.render(), "<div><h1>Hello!</h1><p>Hello world!</p></div>");
```

Because this is so common, there's another macro called `html!` that calls `rsx!` to generate
a component tree, and then calls `render` on it. Most of the time, you'll find yourself using
the `rsx!` macro to compose arbitrary components, and only calling `html!` when you need a
String output, when sending a response or generating a Markdown file.

In Render, attributes and plain strings are escaped using the `render::html_escaping` module. In order to
use un-escaped values so you can dangerously insert raw HTML, use the `raw!` macro around your
string:

```rust
#![feature(proc_macro_hygiene)]

use render::{html, raw};

let tree = html! {
  <div>
    <p>{"<Hello />"}</p>
    <p>{raw!("<Hello />")}</p>
  </div>
};

assert_eq!(tree, "<div><p>&lt;Hello /&gt;</p><p><Hello /></p></div>");
```

### Custom components

Render's greatest ability is to provide type-safety along with custom renderable components.
Introducing new components is as easy as defining a function that returns a `Render` value.

In order to build up components from other components or HTML nodes, you can use the `rsx!`
macro, which generates a `Render` component tree:

```rust
#![feature(proc_macro_hygiene)]

use render::{component, rsx, html};

#[component]
fn Heading<'title>(title: &'title str) {
  rsx! { <h1 class={"title"}>{title}</h1> }
}

let rendered_html = html! {
  <Heading title={"Hello world!"} />
};

assert_eq!(rendered_html, r#"<h1 class="title">Hello world!</h1>"#);
```

If you pay close attention, you see that the function `Heading` is:

* declared with an uppercase. Underneath, it generates a struct with the same name, and
implements the `Render` trait on it.
* does not have a return type. This is because everything is written to a writer, for
performance reasons.

#### Full example

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
