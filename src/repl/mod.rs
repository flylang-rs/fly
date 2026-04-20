use std::{collections::VecDeque, io::Write, sync::Arc};

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal,
};
use flylang_common::source::Source;
use flylang_diagnostics::error::DiagnosticsReport;
use flylang_tte::{Interpreter, control_flow::ControlFlow, object::Value};

use crate::repl::{error::{REPLError, REPLResult}, line_history::LineHistory};

mod line_history;
pub mod error;

pub struct REPL {
    interpreter: Interpreter,
    line_counter: usize,

    line_history: line_history::LineHistory
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

            line_history: LineHistory::new()
        }
    }

    fn show_prompt(&self) {
        print!(":{:<2}   > ", self.line_counter);
    }

    /// Reads line, returns a continuation signal.
    /// false - exit the REPL
    /// true - continue
    pub fn read_line(&mut self) -> ReadlineResult {
        terminal::enable_raw_mode().unwrap();

        self.show_prompt();
        std::io::stdout().flush().unwrap();

        let mut line = String::new();

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
                    terminal::disable_raw_mode().unwrap();
                    return ReadlineResult::Break;
                }

                if key_event.code == KeyCode::Backspace {
                    if line.pop().is_some() {
                        print!("\u{0008} \u{0008}");
                        std::io::stdout().flush().unwrap();
                    }

                    continue;
                }

                if key_event.code == KeyCode::Enter {
                    print!("\n\r");
                    std::io::stdout().flush().unwrap();

                    terminal::disable_raw_mode().unwrap();
                    return ReadlineResult::Value(line);
                }

                if key_event.code == KeyCode::Up {
                    if let Some(ln) = self.line_history.prev() {
                        line = ln.to_string();

                        print!("\r");
                        self.show_prompt();
                        print!("{line}\x1b[K");
                    } else {
                        line.clear();

                        print!("\r");
                        self.show_prompt();
                        print!("\x1b[K");
                    }

                    std::io::stdout().flush().unwrap();
                    
                    continue;

                	// todo!("Implement history navigation on keyup");
                }

                if key_event.code == KeyCode::Down {
                    if let Some(ln) = self.line_history.next() {
                        line = ln.to_string();

                        print!("\r");
                        self.show_prompt();
                        print!("{line}\x1b[K");
                    } else {
                        line.clear();

                        print!("\r");
                        self.show_prompt();
                        print!("\x1b[K");
                    }

                    std::io::stdout().flush().unwrap();
                    
                    continue;
                }

                let key = match key_event.code.as_char() {
                    Some(ch) => ch,
                    None => continue,
                };

                print!("{}", key);
                std::io::stdout().flush().unwrap();

                line.push(key);
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
                                    Ok(cf) => cf.unwrap(),
                                    Err(e) => {
                                        eprintln!("{}", e.render());

                                        continue;
                                    }
                                };

                            if let ControlFlow::Value(Value::String(v)) = stringres {
                                println!("      = {v}");
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
