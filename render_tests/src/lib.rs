#[cfg(test)]
use pretty_assertions::assert_eq;
#[cfg(test)]
use render::{component, html, raw, rsx, Render};

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("ui/fail/*.rs");
}

#[test]
fn works_with_dashes() {
    let value = html! { <div data-id={"myid"} /> };
    assert_eq!(value, r#"<div data-id="myid"/>"#);
}

#[test]
fn works_with_raw() {
    let actual = html! {
        <div>{raw!("<Hello />")}</div>
    };

    assert_eq!(actual, "<div><Hello /></div>");
}

#[test]
fn works_with_raw_ident() {
    let actual = html! {
        <input r#type={"text"} />
    };

    assert_eq!(actual, r#"<input type="text"/>"#);
}

#[test]
fn works_with_keywords() {
    assert_eq!(html! { <input type={"text"} /> }, r#"<input type="text"/>"#);
    assert_eq!(html! { <label for={"me"} /> }, r#"<label for="me"/>"#);
}

#[test]
fn element_ordering() {
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
fn attrs_simple_element() {
    let f = "f";
    assert_eq!(html! { <a f /> }, r#"<a f="f"/>"#);
    assert_eq!(html! { <a class={f} /> }, r#"<a class="f"/>"#);
    assert_eq!(html! { <a class="f" /> }, r#"<a class="f"/>"#);
}

#[test]
fn attrs_custom_element() {
    #[component]
    fn Link<'href>(href: &'href str) {
        rsx! { <a href>{"Go"}</a> }
    }

    let href = "/";
    assert_eq!(html! { <Link href /> }, r#"<a href="/">Go</a>"#);
    assert_eq!(html! { <Link href={href} /> }, r#"<a href="/">Go</a>"#);
    assert_eq!(html! { <Link href="/" /> }, r#"<a href="/">Go</a>"#);
}

#[test]
fn raw_children() {
    #[component]
    fn Link<C: Render>(children: C) {
        rsx! { <a>{children}</a> }
    }

    let go = "Go";
    assert_eq!(html! { <Link>{go}</Link> }, r#"<a>Go</a>"#);
    assert_eq!(
        html! { <Link>Go with the flow.</Link> },
        r#"<a>Go with the flow.</a>"#,
    );
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
