use flylang_common::source::Source;
use flylang_diagnostics::error::DiagnosticsReport;

use crate::common::LoadingError;

pub mod common;
pub mod repl;

fn run_file(source: Source) {
    let ast = match common::parse_source(source) {
        Ok(st) => st,
        Err(LoadingError::AnalyzeFailed) => {
            std::process::exit(1);
        },
        Err(e) => {
            eprintln!("{}", e.render());
            
            std::process::exit(1)
        }
    };

    let interpreter = flylang_tte::Interpreter::new();

    let result = interpreter.execute_script(ast);

    println!("Program finished with result: {result:?}");
}

fn main() -> std::io::Result<()> {
    env_logger::builder()
        .default_format()
        .format_timestamp_millis()
        .init();

    if std::env::args().any(|x| x == "--repl") {
        repl::REPL::new().enter();

        std::process::exit(0);
    }

    let filepath = if let Some(arg) = std::env::args().nth(1) {
        arg
    } else {
        eprintln!("No file specified!");

        std::process::exit(1);
    };

    let source_code = std::fs::read_to_string(&filepath)?;

    run_file(Source::new(filepath, source_code));

    Ok(())
}
