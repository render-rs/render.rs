#![feature(proc_macro_hygiene)]

use render::html::HTML5Doctype;
use render::{component, html, rsx, Renderable};

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
        <>
            <HTML5Doctype />
            <Hello world helo-world yes={1 + 1}>
                <div data-testid={"hey"} hello={"hello"}>{format!("HEY!")}</div>
                {other_value}
            </Hello>
        </>
    };
    value
}

#[component]
pub fn Layout<'a, Children: Renderable>(title: &'a str, children: Children) -> String {
    html! {
        <html>
            <head><title>{title}</title></head>
            <body>
                {children}
            </body>
        </html>
    }
}

#[component]
pub fn SomeComponent(name: String) -> String {
    html! {
        <div>{format!("Hello, {}", name)}</div>
    }
}

#[test]
pub fn verify_works() {
    println!("{}", it_works());
}

#[test]
pub fn works_with_raw() {
    use pretty_assertions::assert_eq;
    use render::raw;

    let actual = html! {
        <div>{raw!("<Hello />")}</div>
    };

    assert_eq!(actual, "<div><Hello /></div>");
}
