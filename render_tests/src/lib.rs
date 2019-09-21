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
