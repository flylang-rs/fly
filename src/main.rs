use std::sync::Arc;

use flylang_common::source::Source;
use flylang_diagnostics::error::DiagnosticsReport;

use crate::common::LoadingError;

pub mod common;
pub mod repl;

fn run_file(source: Source) {
    let source = Arc::new(source);

    let ast = match common::parse_source(Arc::clone(&source)) {
        Ok(st) => st,
        Err(LoadingError::AnalyzeFailed) => {
            std::process::exit(1);
        },
        Err(e) => {
            eprintln!("{}", e.render());
            
            std::process::exit(1)
        }
    };

    let mut interpreter = flylang_tte::Interpreter::new();

    let result = match interpreter.execute_script(source, ast) {
        Ok(res) => res,
        Err(e) => {
            flylang_diagnostics::report_simple_error("Exception occured, showing traceback...");

            for (nr, i) in interpreter.calltrace().iter().enumerate() {
                let addr = if let Some((l, c)) = i.address_line_col {
                    format!(":{l}:{c}")
                } else {
                    String::new()
                };

                flylang_diagnostics::report_simple_error(
                    
                    &format!(
                        "  - #{}: {} in {}{}",
                        nr + 1,
                        i.func_name,
                        i.address_filename,
                        addr
                    )
                );
            }

            eprintln!();

            eprintln!("{}", e.render());

            std::process::exit(1)
        },
    };

    println!("Program finished with result: {result:?}");
}

fn show_help() {
    let prog_name = std::env::args().next().unwrap();

    println!("The Fly programming language.");
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!();

    println!("Usage: {prog_name} [COMMAND] [OPTIONS] <script.fly>...");
    println!();

    println!("Options:");
    println!("  --repl\t\tLaunch Fly REPL");
    println!("  -h, --help\t\tShow this help");
    println!();

    println!("Commands:");
    println!("  To be added soon.");
}

fn main() -> std::io::Result<()> {
    env_logger::builder()
        .default_format()
        .format_timestamp_millis()
        .init();

    // TODO: Use `clap` crate for CLI args parsing.
    if std::env::args().any(|x| x == "-h" || x == "--help") {
        show_help();

        std::process::exit(0);
    }

    if std::env::args().any(|x| x == "--repl") {
        repl::REPL::new().enter();

        std::process::exit(0);
    }

    let filepath = if let Some(arg) = std::env::args().nth(1) {
        arg
    } else {
        flylang_diagnostics::report_simple_error("no file specified!");
        eprintln!();

        show_help();

        std::process::exit(1);
    };

    let source_code = std::fs::read_to_string(&filepath)?;

    run_file(Source::new(filepath, source_code));

    Ok(())
}
