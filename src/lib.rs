pub mod args;
mod files;
mod parse_sentences;
mod prefix_table;
pub mod sentence_generator;
mod server;

use crate::args::Args;
use crate::sentence_generator::SentenceGenerator;
use std::io;

pub enum CliMode {
    Cli,
    Server(u32),
}

pub enum CliError {
    Io(io::Error),
    ServerPortBindingFailed(u32),
}
impl From<io::Error> for CliError {
    fn from(inner: io::Error) -> CliError {
        CliError::Io(inner)
    }
}

pub fn run(args: Args) -> Result<(), CliError> {
    let files = files::get_valid_files(&args.books_path)?;
    let contents = files::read_files(&files)?;
    let sg = make_sentence_generator(args.prefix, &contents);

    match args.mode {
        CliMode::Server(port) => {
            server::new(port, sg)?;
        }
        CliMode::Cli => {
            for _ in 0..args.number_of_sentences {
                println!("{}", sg.get_random_sentence());
            }
        }
    }

    Ok(())
}

fn make_sentence_generator(prefix: u32, contents: &Vec<String>) -> SentenceGenerator {
    let mut sg = SentenceGenerator::new(prefix);
    for content in contents {
        sg.add_text(&content);
    }
    sg
}
