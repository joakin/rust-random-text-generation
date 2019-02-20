use std::fs::{self, DirEntry};
use std::io;

pub fn get_valid_files(path: &str) -> Result<Vec<DirEntry>, io::Error> {
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

pub fn read_files(files: &Vec<DirEntry>) -> Result<Vec<String>, io::Error> {
    let mut contents = Vec::new();
    for file in files {
        let content = fs::read_to_string(file.path())?;
        contents.push(content);
    }
    Ok(contents)
}
