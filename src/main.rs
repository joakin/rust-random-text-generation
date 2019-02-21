use rust_random_text_generation::{args::Args, run, CliError};
use std::process;

fn main() {
    process::exit(match run(Args::new()) {
        Ok(_) => 0,
        Err(cli_error) => {
            match cli_error {
                CliError::Io(inner) => eprintln!("IO Error: {}", inner),
                CliError::ServerPortBindingFailed(port) => {
                    eprintln!("Server Error: Couldn't bind server to port {}", port)
                }
            }
            1
        }
    })
}
