use crate::Renderable;

/// Renders the string
/// > TODO: this should escape HTML!
impl Renderable for String {
    fn render(self) -> String {
        self
    }
}

/// Renders the string
/// > TODO: this should escape HTML!
impl Renderable for &str {
    fn render(self) -> String {
        self.to_string()
    }
}
