use std::fs::{self, DirEntry};
use std::{env, process, thread, time};

use rust_random_text_generation::SentenceGenerator;

const USAGE: &str = "\
    rust_random_text_generation [prefix-length=3] text_files_path

Arguments:
  - prefix-length: Prefix length for the markov chain. Optional, defaults to 3
  - text_files_path: Path to txt files";

fn main() {
    let args: Vec<String> = env::args().collect();

    if !(args.len() == 3 || args.len() == 2) {
        exit(USAGE);
    }

    let (prefix, books_path) = if args.len() == 3 {
        let prefix: u32 = args[1].parse::<u32>().unwrap_or(3);
        let books_path = &args[2];
        (prefix, books_path)
    } else {
        // if args.len() == 2
        (3, &args[1])
    };

    let time_between_sentences = time::Duration::from_millis(1000);
    match make_sentence_generator(prefix, books_path) {
        Err(s) => exit(&s),
        Ok(sg) => loop {
            println!("{}", sg.get_random_sentence());
            thread::sleep(time_between_sentences);
        },
    };
}

fn make_sentence_generator(prefix: u32, books_path: &str) -> Result<SentenceGenerator, String> {
    valid_files(books_path).and_then(|files| {
        let mut sg = SentenceGenerator::new(prefix);
        for file in files {
            match fs::read_to_string(file.path()) {
                Ok(contents) => {
                    sg.add_text(&contents);
                }
                Err(err) => {
                    return Err(format!("Could not read file {:?}\n{}", file.path(), err));
                }
            }
        }
        Ok(sg)
    })
}

fn valid_files(path: &str) -> Result<Vec<DirEntry>, String> {
    fs::read_dir(path)
        .map_err(|e| format!("Failed to read dir: {}", e))
        .map(|dirs| {
            dirs.filter_map(|file| file.ok())
                .filter_map(|file| is_valid_file(file))
                .fold(Vec::new(), |mut files: Vec<DirEntry>, file| {
                    files.push(file);
                    files
                })
        })
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

fn exit(err: &str) {
    eprintln!("{}", err);
    process::exit(1);
}
