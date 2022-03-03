use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::{BufRead, BufReader, self};


pub fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?)
        .lines()
        .collect()
}

#[derive(Clone, Debug)]
pub struct FileHandler {
    public_path: String,
}

impl FileHandler {
    pub fn read_file(file_name: &str) -> Self {
        let mut dir = inner_main().expect("Couldn't find path");
        dir.push(file_name);
        Self {
            public_path: dir.display().to_string(),
        }
    }

    pub fn get_path(&self) -> String {
        self.public_path.clone()
    }
}

pub fn inner_main() -> io::Result<PathBuf> {
    let exe = env::current_exe()?;
    let dir = exe.parent().expect("Executable must be in some directory");
    let dir = dir.join("resources");
    Ok(dir)
}

