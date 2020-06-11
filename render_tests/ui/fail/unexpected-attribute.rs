use render::{component, html, rsx};

#[component]
fn Heading<'title>(title: &'title str) {
    rsx! { <h1>{title}</h1> }
}

fn main() {
    html! { <Heading t={"Hello world!"} /> };
}
