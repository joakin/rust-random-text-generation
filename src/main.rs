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

    let one_second = time::Duration::from_millis(1000);
    match fill_sentence_generator(prefix, books_path) {
        Err(s) => exit(s),
        Ok(sg) => loop {
            println!("{}", sg.get_random_sentence());
            thread::sleep(one_second);
        },
    };
}

fn fill_sentence_generator(prefix: u32, books_path: &str) -> Result<SentenceGenerator, &str> {
    valid_files(books_path).map(|files| {
        let mut sg = SentenceGenerator::new(prefix);
        for file in files {
            match fs::read_to_string(file.path()) {
                Ok(contents) => {
                    println!("Reading {:?}", file.path());
                    sg.add_text(&contents);
                }
                Err(err) => {
                    dbg!(err);
                    eprintln!("Could not read file {:?}", file.path());
                }
            }
        }
        sg
    })
}

fn valid_files(path: &str) -> Result<Vec<DirEntry>, &str> {
    match fs::read_dir(path) {
        Err(_) => Err("Failed to read dir"),
        Ok(dirs) => {
            let mut files = Vec::new();
            for file in dirs {
                match file {
                    Ok(file) => {
                        let path = file.path();
                        let extension = path.extension();
                        match extension {
                            Some(extension) => {
                                if extension == "txt" || extension == "md" {
                                    println!("path: {:?}", path);
                                    files.push(file);
                                }
                            }
                            None => (), // Ignore
                        }
                    }
                    Err(_) => {
                        return Err("Reading file failed");
                    }
                }
            }
            Ok(files)
        }
    }
}

fn exit(err: &str) {
    eprintln!("{}", err);
    process::exit(1);
}
