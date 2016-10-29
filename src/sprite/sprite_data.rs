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

    pub fn add_line<S: Into<String>>(&mut self, line: S) {
        &self.value.push(line.into());
    }

    pub fn len(&self) -> usize {
        self.value.len()
    }

    pub fn line(&self, index: usize) -> String {
        self.value[index].clone()
    }


}
