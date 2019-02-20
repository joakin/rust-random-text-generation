use clap::{App, Arg};
use std::{io, process};

mod files;
use rust_random_text_generation::SentenceGenerator;

const APP_NAME: &'static str = env!("CARGO_PKG_NAME");

struct Args {
    prefix: u32,
    number_of_sentences: u32,
    mode: CliMode,
    books_path: String,
}

enum CliMode {
    Cli,
    Server(u32),
}

enum CliError {
    Io(io::Error),
}
impl From<io::Error> for CliError {
    fn from(inner: io::Error) -> CliError {
        CliError::Io(inner)
    }
}

fn main() {
    process::exit(match run() {
        Ok(_) => 0,
        Err(cli_error) => {
            match cli_error {
                CliError::Io(inner) => eprintln!("IO Error: {}", inner),
            }
            1
        }
    })
}

fn run() -> Result<(), CliError> {
    let args = args();

    let files = files::get_valid_files(&args.books_path)?;
    let contents = files::read_files(&files)?;
    let sg = make_sentence_generator(args.prefix, &contents);

    match args.mode {
        CliMode::Server(_port) => panic!("SERVER MODE NOT IMPLEMENTED YET"),
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
        sg.add_text(content);
    }
    sg
}

fn args() -> Args {
    let matches = App::new(APP_NAME)
        .about("Generates random sentences out of a folder with txt books")
        .arg(
            Arg::with_name("prefix_length")
                .short("p")
                .long("prefix-length")
                .value_name("NUMBER")
                .help("Set the number of words to look into to find the next word.")
                .takes_value(true)
                .default_value("3")
                .validator(is_natural_number),
        )
        .arg(
            Arg::with_name("number_of_sentences")
                .short("n")
                .long("number-of-sentences")
                .value_name("NUMBER")
                .help("Set the number of sentences to get at once, separated by newlines. Ignored in server mode.")
                .takes_value(true)
                .default_value("1")
                .validator(is_natural_number),
        )
        .arg(
            Arg::with_name("server")
                .long("server")
                .short("s")
                .value_name("PORT")
                .help("Run the application in HTTP server mode")
                .takes_value(true)
                .validator(is_natural_number),
        )
        .arg(
            Arg::with_name("folder")
                .index(1)
                .value_name("BOOKS_FOLDER")
                .help("Path to the folder that contains the books")
                .required(true),
        )
        .get_matches();

    let prefix = matches
        .value_of("prefix_length")
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let number_of_sentences = matches
        .value_of("number_of_sentences")
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let mode = match matches.value_of("server") {
        Some(port) => CliMode::Server(port.parse::<u32>().unwrap()),
        None => CliMode::Cli,
    };

    let books_path = matches.value_of("folder").unwrap().to_string();

    Args {
        prefix,
        number_of_sentences,
        mode,
        books_path,
    }
}

fn is_natural_number(v: String) -> Result<(), String> {
    v.parse::<u32>()
        .map(|_| ())
        .map_err(|_| String::from("Should be a natural number"))
}
