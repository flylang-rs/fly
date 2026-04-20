pub struct LineHistory {
    container: Vec<String>,
    index: usize
}

impl LineHistory {
    pub fn new() -> Self {
        Self {
            container: vec![],
            index: 0,
        }
    }

    fn update_index(&mut self) {
        self.index = self.container.len();
    }

    pub fn push(&mut self, line: String) {
        self.container.push(line);

        self.update_index();
    }

    pub fn next(&mut self) -> Option<&str> {
        let value = self.container.get(self.index + 1);

        if self.index < self.container.len() {
            self.index += 1;
        }

        value.map(|x| x.as_str())
    }

    pub fn prev(&mut self) -> Option<&str> {
        let value = self.container.get(self.index.saturating_sub(1));

        self.index = self.index.saturating_sub(1);

        value.map(|x| x.as_str())
    }
}