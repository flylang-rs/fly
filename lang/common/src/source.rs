/// Describes the source file with code.
#[derive(Debug, Clone)]
pub struct Source {
    pub filepath: String,
    pub code: String,

    line_starts: Vec<usize>, // byte offset of the first char on each line
}

impl Source {
    pub fn new(filepath: String, code: String) -> Self {
        // Track byte position of every line start.

        // How that works: `once` will yield a zero for the firsi iteration,
        // then we connect (chain) another iterator that tracks byte position of line starts.
        let line_starts = std::iter::once(0)
            .chain(
                code.char_indices()
                    .filter(|(_, c)| *c == '\n')
                    .map(|(i, _)| i + 1),
            )
            .collect();

        Self {
            filepath,
            code,
            line_starts,
        }
    }

    /// Returns (line, column), both 1-indexed.
    pub fn location(&self, byte_offset: usize) -> (usize, usize) {
        let line_idx = self
            .line_starts
            .partition_point(|&start| start <= byte_offset)
            - 1;
        let col = byte_offset - self.line_starts[line_idx];
        (line_idx + 1, col + 1)
    }

    /// Returns the full text of a given line (1-indexed), without the newline.
    pub fn line_text(&self, line: usize) -> &str {
        let start = self.line_starts[line - 1];
        let end = self
            .line_starts
            .get(line)
            .copied()
            .unwrap_or(self.code.len());

        self.code[start..end].trim_end_matches('\n')
    }

    /// Oh gosh...
    pub fn line_start(&self, line: usize) -> usize {
        *self.line_starts.get(line).unwrap()
    }
}
