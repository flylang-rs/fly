use flylang_common::Address;

use owo_colors::{OwoColorize, Stream};

use crate::{
    additions::{Help, Note, TextEdit},
    kind::DiagnosticsKind,
};

pub mod additions;
pub mod kind;

pub struct Diagnostics {
    // ...
}

impl Diagnostics {
    fn transform_lines_to_diag_part(code: &str) -> String {
        let mut result = String::with_capacity(code.len());

        for line in code.lines() {
            result.push_str("     | ");
            result.push_str(line);
            result.push('\n');
        }

        // Pop the last \n
        result.pop();

        result
    }

    fn apply_edits(source_line: &str, edits: &[TextEdit], line_start_offset: usize) -> String {
        let mut result = source_line.to_string();

        // Apply edits in reverse order so offsets stay valid
        let mut edits_sorted = edits.to_vec();
        edits_sorted.sort_by(|a, b| b.span.start.cmp(&a.span.start));

        for edit in &edits_sorted {
            let local_start = edit.span.start - line_start_offset;
            let local_end = edit.span.end - line_start_offset;

            let replacement = &edit.replacement.as_deref().unwrap_or("");

            result.replace_range(local_start..local_end, replacement);
        }

        result
    }

    /// Makes a (maube colored) diff for help sections in diagnostic.
    fn make_diff(source_line: &str, edits: &[TextEdit], line_start_offset: usize) -> String {
        let mut result = String::new();
        let positions = diff::chars(
            source_line,
            &Self::apply_edits(source_line, edits, line_start_offset),
        );

        for i in &positions {
            let (ch, color) = match i {
                diff::Result::Left(l) => (l, owo_colors::AnsiColors::BrightRed),
                diff::Result::Both(l, _) => (l, owo_colors::AnsiColors::Default),
                diff::Result::Right(r) => (r, owo_colors::AnsiColors::BrightGreen),
            };

            result.push_str(&ch.color(color).to_string());
        }

        result.push('\n');

        for i in &positions {
            let (ch, color) = match i {
                diff::Result::Left(_) => ('-', owo_colors::AnsiColors::BrightRed),
                diff::Result::Both(_, _) => (' ', owo_colors::AnsiColors::Default),
                diff::Result::Right(_) => ('+', owo_colors::AnsiColors::BrightGreen),
            };

            result.push_str(&ch.color(color).to_string());
        }

        result
    }

    fn emit(
        &self,
        kind: DiagnosticsKind,
        message: &str,
        address: &Address,
        notes: &[Note],
        helps: &[Help],
    ) {
        let src = &address.source;
        let location = src.location(address.span.start);
        let code_line = src.line_text(location.0);

        eprintln!(
            "{}: {}:{}:{}: {} ",
            kind.as_str()
                .if_supports_color(Stream::Stderr, |x| x.color(kind.color()))
                .if_supports_color(Stream::Stderr, |x| x.bold()),
            src.filepath,
            location.0,
            location.1,
            message.if_supports_color(Stream::Stderr, |x| x.bold())
        );
        eprintln!("     |");
        eprintln!("{:>4} | {}", location.0, code_line);

        for i in notes {
            let location = src.location(i.position.span.start);

            eprintln!(
                "     | {}{} {}",
                " ".repeat(location.1 - 1),
                "^".repeat(i.position.span.end - i.position.span.start),
                i.message.if_supports_color(Stream::Stderr, |x| x.bold())
            );
        }

        if !helps.is_empty() {
            eprintln!("     |");
        }

        for (i, n) in helps.iter().enumerate() {
            eprintln!(
                "`- {} nr. {}: {}\n     |\n{}",
                "help"
                    .if_supports_color(Stream::Stderr, |x| x.green())
                    .if_supports_color(Stream::Stderr, |x| x.bold()),
                i + 1,
                n.message.if_supports_color(Stream::Stderr, |x| x.bold()),
                Self::transform_lines_to_diag_part(&Self::make_diff(
                    code_line,
                    &n.edits,
                    src.line_start(location.0 - 1)
                ))
            );
        }

        eprintln!("     |");
        eprintln!();
    }

    pub fn error(&self, error: &str, address: &Address, notes: &[Note], helps: &[Help]) {
        self.emit(DiagnosticsKind::Error, error, address, notes, helps);
    }

    pub fn warning(&self, warning: &str, address: &Address, notes: &[Note], helps: &[Help]) {
        self.emit(DiagnosticsKind::Warning, warning, address, notes, helps);
    }
}
