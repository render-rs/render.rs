#![feature(proc_macro_hygiene)]

#[test]
pub fn works_with_dashes() {
    use pretty_assertions::assert_eq;

    let value = render::html! { <div data-id={"myid"} /> };
    assert_eq!(value, r#"<div data-id="myid"/>"#);
}

#[test]
pub fn works_with_raw() {
    use pretty_assertions::assert_eq;
    use render::{html, raw};

    let actual = html! {
        <div>{raw!("<Hello />")}</div>
    };

    assert_eq!(actual, "<div><Hello /></div>");
}

mod kaki {
    // A simple HTML 5 doctype declaration
    use render::html::HTML5Doctype;
    use render::{
        // A macro to create components
        component,
        // A macro to compose components in JSX fashion
        rsx,
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

    #[test]
    fn test() {
        use pretty_assertions::assert_eq;
        let actual = render::html! {
          <Page title={"Home"}>
            {format!("Welcome, {}", "Gal")}
          </Page>
        };
        let expected = concat!(
            "<!DOCTYPE html>",
            "<html>",
            "<head><title>Home</title></head>",
            "<body>",
            "Welcome, Gal",
            "</body>",
            "</html>"
        );
        assert_eq!(actual, expected);
    }
}
