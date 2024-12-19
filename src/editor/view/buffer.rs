#[derive(Default)]
pub struct Buffer {
    pub contents: Vec<String>,
}

impl Buffer {
    pub fn is_empty(&self) -> bool {
        self.contents.is_empty()
    }
}
