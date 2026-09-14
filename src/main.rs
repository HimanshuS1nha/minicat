use clap::Parser;
use minicat::{Args, run};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = Args::parse();

    match run(args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("minicat: {error}");
            ExitCode::FAILURE
        }
    }
}
