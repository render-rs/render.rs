/// A renderable component
pub trait Renderable: core::fmt::Debug + Sized {
    /// Render the component to the HTML representation.
    ///
    /// Mostly done using the `html!` macro to generate strings
    /// by composing tags.
    ///
    /// A simple implementation:
    ///
    /// ```rust
    /// # #![feature(proc_macro_hygiene)]
    /// # use render_macros::html;
    /// # use render::Renderable;
    /// # use pretty_assertions::assert_eq;
    /// #[derive(Debug)]
    /// struct Header<'t> { title: &'t str }
    ///
    /// impl<'t> Renderable for Header<'t> {
    ///     fn render(self) -> String {
    ///         html! {
    ///             <h1>{self.title}</h1>
    ///         }
    ///     }
    /// }
    ///
    /// // Then you can use it with
    ///
    /// let rendered_html = html! {
    ///     <Header title={"Hello world!"} />
    /// };
    ///
    /// # assert_eq!(rendered_html, "<h1>Hello world!</h1>");
    /// ```
    ///
    /// ## Children
    ///
    /// `children` is a special field that will be populated with other `Renderable` if any children was provided, by both the [`html!`] and the [`rsx!`] macros.
    ///
    /// [`html!`]: ../render_macros/macro.html.html
    /// [`rsx!`]: ../render_macros/macro.rsx.html
    fn render(self) -> String;
}

/// Renders an empty string
impl Renderable for () {
    fn render(self) -> String {
        "".to_string()
    }
}

/// Renders `A` and then `B`
impl<A: Renderable, B: Renderable> Renderable for (A, B) {
    fn render(self) -> String {
        format!("{}{}", self.0.render(), self.1.render())
    }
}

/// Renders `A`, `B`, and then `C`
impl<A: Renderable, B: Renderable, C: Renderable> Renderable for (A, B, C) {
    fn render(self) -> String {
        ((self.0, self.1), self.2).render()
    }
}

/// Renders `A`, `B`, `C` and then `D`
impl<A: Renderable, B: Renderable, C: Renderable, D: Renderable> Renderable for (A, B, C, D) {
    fn render(self) -> String {
        ((self.0, self.1), (self.2, self.3)).render()
    }
}

/// Renders the `T` or an empty string
impl<T: Renderable> Renderable for Option<T> {
    fn render(self) -> String {
        match self {
            None => "".to_string(),
            Some(x) => x.render(),
        }
    }
}

/// Renders `O` or `E`
impl<O: Renderable, E: Renderable> Renderable for Result<O, E> {
    fn render(self) -> String {
        match self {
            Err(e) => e.render(),
            Ok(o) => o.render(),
        }
    }
}

impl Renderable for usize {
    fn render(self) -> String {
        self.to_string()
    }
}
