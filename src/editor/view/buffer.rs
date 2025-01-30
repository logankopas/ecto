use std::io::Error;
use std::fs::read_to_string;

#[derive(Default)]
pub struct Buffer {
    pub data: Vec<String>
}

impl Buffer {
    pub fn load(filename: &str) -> Result<Self, Error> {
        let file_contents = read_to_string(filename)?;
        let mut contents = Vec::new();
        for line in file_contents.lines() {
            contents.push(line.to_string());
        }
        Ok(Self { data: contents })
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
