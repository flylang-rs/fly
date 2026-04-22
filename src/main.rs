use log::info;

use std::sync::Arc;

use flylang_common::source::Source;
use flylang_diagnostics::error::DiagnosticsReport;
use flylang_lexparse_glue::LoadingError;

use crate::arguments::CommandLineArguments;

pub mod arguments;
pub mod repl;

fn run_file(options: &CommandLineArguments, source: Source) {
    let source = Arc::new(source);

    if options.show_lexems {
        let tokens = flylang_lexparse_glue::lex_source(Arc::clone(&source)).map(|x| {
            x.iter()
                .map(|y| (y.value.clone(), y.address.span.clone()))
                .collect::<Vec<_>>()
        });

        println!("{:?}", tokens);
    }

    let ast = match flylang_lexparse_glue::parse_source(Arc::clone(&source)) {
        Ok(st) => st,
        Err(LoadingError::AnalyzeFailed) => {
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("{}", e.render());

            std::process::exit(1)
        }
    };

    if options.show_ast {
        eprintln!("{ast:#?}");
    }

    let mut interpreter = flylang_tte::Interpreter::new();

    let result = match interpreter.execute_script(ast) {
        Ok(res) => res,
        Err(e) => {
            flylang_diagnostics::report_simple_error("Exception occured, showing traceback...");

            for (nr, i) in interpreter.calltrace().iter().enumerate() {
                let addr = if let Some((l, c)) = i.call_site.address_line_col {
                    format!(":{l}:{c}")
                } else {
                    String::new()
                };

                flylang_diagnostics::report_simple_error(&format!(
                    "  - #{}: {} called from {}{} ({})",
                    nr + 1,
                    i.function_name,
                    i.call_site.address_filename,
                    addr,
                    i.from.as_deref().unwrap_or("???")
                ));
            }

            eprintln!();

            eprintln!("{}", e.render());

            std::process::exit(1)
        }
    };

    info!("Program finished with result: {result:?}");
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
    println!("  --show-ast\t\tShow AST");
    println!("  --show-lexems\t\tShow token stream");
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

    let mut cmd_opts = CommandLineArguments::default();

    // TODO: Use `clap` crate for CLI args parsing.
    if std::env::args().any(|x| x == "-h" || x == "--help") {
        show_help();

        std::process::exit(0);
    }

    if std::env::args().any(|x| x == "--repl") {
        repl::REPL::new().enter();

        std::process::exit(0);
    }

    if std::env::args().any(|x| x == "--show-ast") {
        cmd_opts.show_ast = true;
    }

    if std::env::args().any(|x| x == "--show-lexems") {
        cmd_opts.show_lexems = true;
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

    run_file(&cmd_opts, Source::new(filepath, source_code));

    Ok(())
}
