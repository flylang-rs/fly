use std::{io::Write, sync::Arc};

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal,
};
use flylang_common::source::Source;
use flylang_diagnostics::error::DiagnosticsReport;
use flylang_tte::{Interpreter, control_flow::ControlFlow, object::Value};

use crate::repl::{
    error::{REPLError, REPLResult},
    formatter::REPLFormatter,
    line_history::LineHistory,
};

pub mod error;
mod formatter;
mod line_history;

pub struct REPL {
    interpreter: Interpreter,
    line_counter: usize,

    line_history: line_history::LineHistory,
    cursor_position: usize, // Relative to text, not the whole prompt
}

pub enum ReadlineResult {
    Ignore,
    Break,
    Value(String),
}

impl REPL {
    pub fn new() -> Self {
        Self {
            interpreter: Interpreter::new(),
            line_counter: 1,

            line_history: LineHistory::new(),
            cursor_position: 0,
        }
    }

    fn show_prompt(&self) {
        print!(":{:<2}   > ", self.line_counter);
    }

    fn format_line(&self, line: &str) {
        // Do a simple split here
        /*
        let stems = Self::split_and_keep(line);

        for i in stems.iter() {
            if flylang_lexer::kw_lookup_table::KEYWORDS.contains_key(i) {
                print!(
                    "{}",
                    i.if_supports_color(Stream::Stdout, |x| x.bold())
                        .if_supports_color(Stream::Stdout, |x| x.bright_magenta())
                );

                continue;
            }

            print!("{}", i);
        }
        */

        let formatted = REPLFormatter::format(line);
        let ln = formatted.as_deref().unwrap_or(line);

        print!("{ln}");
    }

    fn redraw(&self, line: &[char], cursor: usize) {
        print!("\r");

        self.show_prompt();
        self.format_line(&line.into_iter().collect::<String>());

        print!("\x1b[K");

        print!("\x1b[{}D", line.len() + 1);
        print!("\x1b[{}C", cursor + 1);

        std::io::stdout().flush().unwrap();
    }

    /// Reads line, returns a continuation signal.
    /// false - exit the REPL
    /// true - continue
    pub fn read_line(&mut self) -> ReadlineResult {
        terminal::enable_raw_mode().unwrap();

        self.show_prompt();
        std::io::stdout().flush().unwrap();

        let mut line: Vec<char> = vec![];

        self.cursor_position = 0;

        loop {
            if let Event::Key(key_event) = event::read().unwrap() {
                if key_event.modifiers.contains(KeyModifiers::CONTROL)
                    && key_event.code == KeyCode::Char('c')
                {
                    print!("\n\r");
                    break;
                }

                if key_event.modifiers.contains(KeyModifiers::CONTROL)
                    && key_event.code == KeyCode::Char('d')
                {
                    print!("\r\n");
                    terminal::disable_raw_mode().unwrap();
                    return ReadlineResult::Break;
                }

                if key_event.code == KeyCode::Backspace {
                    if self.cursor_position == 0 {
                        continue;
                    }

                    if self.cursor_position <= line.len() {
                        line.remove(self.cursor_position - 1);

                        self.cursor_position -= 1;

                        self.redraw(&line, self.cursor_position);
                    }

                    continue;
                }

                if key_event.code == KeyCode::Enter {
                    print!("\n\r");
                    std::io::stdout().flush().unwrap();

                    terminal::disable_raw_mode().unwrap();

                    return ReadlineResult::Value(line.into_iter().collect::<String>());
                }

                if key_event.code == KeyCode::Up {
                    if let Some(ln) = self.line_history.prev() {
                        line = ln.chars().collect();
                        self.cursor_position = line.len();

                        self.redraw(&line, self.cursor_position);
                    } else {
                        line.clear();
                        self.cursor_position = 0;

                        self.redraw(&line, self.cursor_position);
                    }

                    std::io::stdout().flush().unwrap();

                    continue;
                }

                if key_event.code == KeyCode::Down {
                    if let Some(ln) = self.line_history.next() {
                        line = ln.chars().collect();
                        self.cursor_position = line.len();

                        self.redraw(&line, self.cursor_position);
                    } else {
                        line.clear();
                        self.cursor_position = 0;

                        self.redraw(&line, self.cursor_position);
                    }

                    std::io::stdout().flush().unwrap();

                    continue;
                }

                if key_event.code == KeyCode::Left {
                    self.cursor_position = self.cursor_position.saturating_sub(1);

                    self.redraw(&line, self.cursor_position);

                    continue;
                }

                if key_event.code == KeyCode::Right {
                    if self.cursor_position >= line.len() {
                        continue;
                    }

                    self.cursor_position += 1;

                    self.redraw(&line, self.cursor_position);

                    continue;
                }

                if key_event.code == KeyCode::Home {
                    self.cursor_position = 0;

                    self.redraw(&line, self.cursor_position);

                    continue;
                }

                if key_event.code == KeyCode::End {
                    self.cursor_position = line.len();

                    self.redraw(&line, self.cursor_position);

                    continue;
                }

                let key = match key_event.code.as_char() {
                    Some(ch) => ch,
                    None => continue,
                };

                line.insert(self.cursor_position, key);

                self.cursor_position += 1;

                self.redraw(&line, self.cursor_position);
            }
        }

        terminal::disable_raw_mode().unwrap();

        ReadlineResult::Ignore
    }

    pub fn execute(&mut self, line: String) -> REPLResult<ControlFlow> {
        let ast = flylang_lexparse_glue::parse_source(Arc::new(Source::new(
            String::from("<REPL>"),
            line,
        )))
        .map_err(REPLError::ModuleLoadingError)?;

        let result = self
            .interpreter
            .execute(ast)
            .map_err(REPLError::InterpreterError)?;

        Ok(result)
    }

    pub fn enter(&mut self) {
        println!("Fly REPL (version: {})", env!("CARGO_PKG_VERSION"));
        println!("Hit Ctrl-D or use `exit()` to exit.");
        println!();

        loop {
            let ln = self.read_line();

            match ln {
                ReadlineResult::Ignore => continue,
                ReadlineResult::Break => break,
                ReadlineResult::Value(line) => {
                    self.line_history.push(line.clone());

                    let result = self.execute(line);

                    match result {
                        Err(e) => {
                            eprintln!("{}", e.render());
                        }
                        Ok(ControlFlow::Value(val)) => {
                            let ty = flylang_tte::types::value_to_internal_type(&val).unwrap();

                            let methodname = format!("{ty}::to_displayable");

                            let stringres =
                                match self.interpreter.call_func_extern(&methodname, &[val]) {
                                    Ok(cf) => cf,
                                    Err(e) => {
                                        eprintln!("{}", e.render());

                                        continue;
                                    }
                                };

                            if let Some(cf) = stringres {
                                if let ControlFlow::Value(Value::String(v)) = cf {
                                    let output_fmt =
                                        REPLFormatter::format(&v).unwrap_or_else(|_| v.to_string());

                                    println!("      = {output_fmt}");
                                } else {
                                    panic!("Expected string value, got: {cf:?}");
                                }
                            }
                        }
                        Ok(ControlFlow::Nothing) => (),
                        _ => panic!("Don't know what to show for CF = {result:?}"),
                    }

                    self.line_counter += 1;
                }
            }
        }
    }
}
