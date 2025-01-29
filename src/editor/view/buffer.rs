pub struct Buffer {
    pub data: Vec<String>
}

impl Default for Buffer {
    fn default() -> Self {
        Self {
            data: vec![String::from("Hello, friends!")]
        }
    }
}
