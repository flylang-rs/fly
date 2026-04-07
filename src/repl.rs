use std::{
    io::{Write},
};

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal,
};
use flylang_common::source::Source;
use flylang_diagnostics::{error::DiagnosticsReport};
use flylang_tte::{Interpreter, control_flow::ControlFlow, object::Value};

use crate::common::LoadingResult;

pub struct REPL {
    interpreter: Interpreter,
    line_counter: usize
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
            line_counter: 1
        }
    }

    /// Reads line, returns a continuation signal.
    /// false - exit the REPL
    /// true - continue
    pub fn read_line(&mut self) -> ReadlineResult {
        terminal::enable_raw_mode().unwrap();

        write!(std::io::stdout(), ":{:<2}   > ", self.line_counter).unwrap();
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

    pub fn execute(&mut self, line: String) -> LoadingResult<ControlFlow> {
        let ast = crate::common::parse_source(Source::new(String::from("<REPL>"), line))?;

        Ok(self.interpreter.execute(ast))
    }

    pub fn enter(&mut self) {
        println!("Fly REPL (version: {})", env!("CARGO_PKG_VERSION"));
        println!("Hit Ctrl-D to exit.");
        println!();

        loop {
            let ln = self.read_line();

            match ln {
                ReadlineResult::Ignore => continue,
                ReadlineResult::Break => break,
                ReadlineResult::Value(line) => {
                    let result = self.execute(line);

                    match result {
                        Err(e) => {
                            eprintln!("{}", e.render());
                        }
                        Ok(ControlFlow::Value(val)) => {
                            let ty = flylang_tte::types::value_to_internal_type(&val).unwrap();

                            let methodname = format!("{ty}::to_displayable");

                            let stringres = self.interpreter.call_func_extern(&methodname, &[val]).unwrap();

                            if let ControlFlow::Value(Value::String(v)) = stringres {
                                println!("      = {v}");
                            }
                        }
                        Ok(ControlFlow::Nothing) => (),
                        _ => panic!("Don't know what to show for CF = {result:?}")
                    }

                    self.line_counter += 1;
                }
            }
        }

    }
}
