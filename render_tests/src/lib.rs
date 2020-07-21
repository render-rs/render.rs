#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("ui/fail/*.rs");
}

#[test]
fn works_with_dashes() {
    use pretty_assertions::assert_eq;

    let value = render::html! { <div data-id={"myid"} /> };
    assert_eq!(value, r#"<div data-id="myid"/>"#);
}

#[test]
fn works_with_raw() {
    use pretty_assertions::assert_eq;
    use render::{html, raw};

    let actual = html! {
        <div>{raw!("<Hello />")}</div>
    };

    assert_eq!(actual, "<div><Hello /></div>");
}

#[test]
fn works_with_raw_ident() {
    use pretty_assertions::assert_eq;

    let actual = render::html! {
        <input r#type={"text"} />
    };

    assert_eq!(actual, r#"<input type="text"/>"#);
}

#[test]
fn works_with_keywords() {
    use pretty_assertions::assert_eq;
    use render::html;

    assert_eq!(html! { <input type={"text"} /> }, r#"<input type="text"/>"#);
    assert_eq!(html! { <label for={"me"} /> }, r#"<label for="me"/>"#);
}

#[test]
fn element_ordering() {
    use pretty_assertions::assert_eq;
    use render::html;

    let actual = html! {
      <ul>
        <li>{"1"}</li>
        <li>{"2"}</li>
        <li>{"3"}</li>
      </ul>
    };

    assert_eq!(actual, "<ul><li>1</li><li>2</li><li>3</li></ul>");

    let deep = html! {
      <div>
        <h1>{"A list"}</h1>
        <hr />
        <ul>
          <li>{"1"}</li>
          <li>{"2"}</li>
          <li>{"3"}</li>
        </ul>
      </div>
    };

    assert_eq!(
        deep,
        "<div><h1>A list</h1><hr/><ul><li>1</li><li>2</li><li>3</li></ul></div>"
    );
}

#[test]
fn some_none() {
    use pretty_assertions::assert_eq;
    use render::{component, html, rsx};

    #[component]
    fn Answer(a: i8) {
        rsx! {
          <>
            {match a {
              42 => Some("Yes"),
              _ => None,
            }}
          </>
        }
    }

    assert_eq!(html! { <Answer a={42} /> }, "Yes");
    assert_eq!(html! { <Answer a={44} /> }, "");
}

#[test]
fn owned_string() {
    use pretty_assertions::assert_eq;
    use render::{component, html, rsx};

    #[component]
    fn Welcome<'kind, 'name>(kind: &'kind str, name: &'name str) {
        rsx! {
            <h1 class={format!("{}-title", kind)}>
                {format!("Hello, {}", name)}
            </h1>
        }
    }

    assert_eq!(
        html! { <Welcome kind={"alien"} name={"Yoda"} /> },
        r#"<h1 class="alien-title">Hello, Yoda</h1>"#
    );
}

#[test]
fn number() {
    use pretty_assertions::assert_eq;
    use render::html;

    let num = 42;

    assert_eq!(html! { <p>{num}</p> }, "<p>42</p>")
}

#[test]
fn vec() {
    use pretty_assertions::assert_eq;
    use render::html;

    let list = vec!["Mouse", "Rat", "Hamster"];

    assert_eq!(
        html! {
            <ul>
                {
                    list
                        .into_iter()
                        .map(|text| render::rsx! { <li>{text}</li> })
                        .collect::<Vec<_>>()
                }
            </ul>
        },
        "<ul><li>Mouse</li><li>Rat</li><li>Hamster</li></ul>"
    )
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

    #[test]
    fn externals_test() {
        use crate::other::ExternalPage;
        use pretty_assertions::assert_eq;

        let actual = render::html! {
          <ExternalPage title={"Home"} subtitle={"Foo"}>
            {format!("Welcome, {}", "Gal")}
          </ExternalPage>
        };

        let expected = concat!(
            "<!DOCTYPE html>",
            "<html>",
            "<head><title>Home</title></head>",
            "<body>",
            "<h1>Foo</h1>",
            "Welcome, Gal",
            "</body>",
            "</html>"
        );
        assert_eq!(actual, expected);
    }
}

/// ## Other
///
/// Module for testing component visibility when imported from other modules.

mod other {
    use render::html::HTML5Doctype;
    use render::{component, rsx, Render};

    #[component]
    pub fn ExternalPage<'title, 'subtitle, Children: Render>(
        title: &'title str,
        subtitle: &'subtitle str,
        children: Children,
    ) {
        rsx! {
            <>
              <HTML5Doctype />
              <html>
                <head><title>{title}</title></head>
                <body>
                  <h1>{subtitle}</h1>
                  {children}
                </body>
              </html>
            </>
        }
    }
}
