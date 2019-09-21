use crate::Renderable;

impl Renderable for String {
    fn render(self) -> String {
        self
    }
}

impl Renderable for &str {
    fn render(self) -> String {
        self.to_string()
    }
}
