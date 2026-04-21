/// Implements history feature.
/// It contains all lines were interpreted, and index for navigation (cursor).
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

    /// Pushes a line into the history and sets cursor to the end.
    pub fn push(&mut self, line: String) {
        self.container.push(line);

        self.update_index();
    }

    /// Moves cursor on next position, if possible.
    /// Returns a line that placed after cursor move.
    pub fn next(&mut self) -> Option<&str> {
        let value = self.container.get(self.index + 1);

        if self.index < self.container.len() {
            self.index += 1;
        }

        value.map(|x| x.as_str())
    }

    /// Moves cursor on previous position, if possible.
    /// Returns a line that placed after cursor move.
    pub fn prev(&mut self) -> Option<&str> {
        let value = self.container.get(self.index.saturating_sub(1));

        self.index = self.index.saturating_sub(1);

        value.map(|x| x.as_str())
    }
}