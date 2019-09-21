/// A renderable component
pub trait Renderable: core::fmt::Debug + Sized {
    /// Render the component to the HTML representation.
    ///
    /// Mostly done using the `html!` macro to generate strings
    /// by composing tags.
    fn render(self) -> String;
}

impl Renderable for () {
    fn render(self) -> String {
        "".to_string()
    }
}

impl<A: Renderable, B: Renderable> Renderable for (A, B) {
    fn render(self) -> String {
        format!("{}{}", self.0.render(), self.1.render())
    }
}

impl<A: Renderable, B: Renderable, C: Renderable> Renderable for (A, B, C) {
    fn render(self) -> String {
        ((self.0, self.1), self.2).render()
    }
}

impl<A: Renderable, B: Renderable, C: Renderable, D: Renderable> Renderable for (A, B, C, D) {
    fn render(self) -> String {
        ((self.0, self.1), (self.2, self.3)).render()
    }
}

impl<T: Renderable> Renderable for Option<T> {
    fn render(self) -> String {
        match self {
            None => "".to_string(),
            Some(x) => x.render(),
        }
    }
}
