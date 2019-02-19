use std::fs::{self, DirEntry};
use std::{env, io, process};

use rust_random_text_generation::SentenceGenerator;

fn main() {
    process::exit(match run() {
        Ok(_) => 0,
        Err(kind) => {
            match kind {
                CliError::InvalidNumberOfArgs => print_usage(),
                CliError::PrefixArgMustBeNumber => {
                    eprintln!("The prefix argument should be a natural number");
                    print_usage();
                }
                CliError::Io(inner) => eprintln!("{}: {}", APP_NAME, inner),
            }
            1
        }
    })
}

fn run() -> Result<(), CliError> {
    let args: Vec<String> = env::args().collect();

    let (prefix, books_path) = if args.len() == 3 {
        let prefix: u32 = args[1]
            .parse::<u32>()
            .map_err(|_| CliError::PrefixArgMustBeNumber)?;
        let books_path = &args[2];
        (prefix, books_path)
    } else if args.len() == 2 {
        (3, &args[1])
    } else {
        return Err(CliError::InvalidNumberOfArgs);
    };

    let files = get_valid_files(books_path)?;
    let contents = read_files(&files)?;
    let sg = make_sentence_generator(prefix, &contents);

    println!("{}", sg.get_random_sentence());

    Ok(())
}

enum CliError {
    Io(io::Error),
    InvalidNumberOfArgs,
    PrefixArgMustBeNumber,
}
impl From<io::Error> for CliError {
    fn from(inner: io::Error) -> CliError {
        CliError::Io(inner)
    }
}

const APP_NAME: &'static str = env!("CARGO_PKG_NAME");

fn print_usage() {
    eprintln!(
        "\
    {} [prefix-length=3] text_files_folder_path

Arguments:
  - prefix-length: Prefix length for the markov chain. Optional, defaults to 3
  - text_files_folder_path: Path to folder with txt files",
        APP_NAME
    );
}

fn get_valid_files(path: &str) -> Result<Vec<DirEntry>, CliError> {
    let entries = fs::read_dir(path)?;
    Ok(entries
        .filter_map(|file| file.ok())
        .filter_map(|file| is_valid_file(file))
        .collect())
}

fn is_valid_file(file: DirEntry) -> Option<DirEntry> {
    let path = file.path();
    path.extension().and_then(|extension| {
        if extension == "txt" || extension == "md" {
            Some(file)
        } else {
            None
        }
    })
}

fn read_files(files: &Vec<DirEntry>) -> Result<Vec<String>, CliError> {
    let mut contents = Vec::new();
    for file in files {
        let content = fs::read_to_string(file.path())?;
        contents.push(content);
    }
    Ok(contents)
}

fn make_sentence_generator(prefix: u32, contents: &Vec<String>) -> SentenceGenerator {
    let mut sg = SentenceGenerator::new(prefix);
    for content in contents {
        sg.add_text(content);
    }
    sg
}
