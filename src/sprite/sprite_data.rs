#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SpriteData {
    value: Vec<String>,
}

impl SpriteData {
    pub fn new() -> SpriteData {
        let value = Vec::new();
        SpriteData {
            value: value,
        }
    }

    pub fn value(&self) -> &Vec<String> {
        &self.value
    }

    pub fn add_line(&mut self, line: String) {
        &self.value.push(line);
    }
}
