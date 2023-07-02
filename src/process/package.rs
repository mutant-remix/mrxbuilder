use std::{path::PathBuf, fs, io::ErrorKind::NotFound};
use crate::load::manifest::Container;

pub struct Package {
    kind: Container,
    path: PathBuf,
}

impl Package {
    pub fn new(kind: &Container, path: &PathBuf) -> Self {
        match fs::remove_dir_all(path) {
            Ok(_) => {}
            Err(err) => {
                if err.kind() != NotFound {
                    panic!("Failed to remove old directory '{}': {}", path.display(), err);
                }
            }
        }

        Self {
            kind: kind.clone(),
            path: path.clone(),
        }
    }

    pub fn add_file(&mut self, file: &Vec<u8>, filename: &str) {
        match self.kind {
            Container::Zip => {
                unimplemented!("Zip support coming soon");
            },
            Container::Directory => {
                let path = self.path.join(filename);
                let dir = path.parent().unwrap();

                match fs::create_dir_all(dir) {
                    Ok(_) => {}
                    Err(err) => {
                        panic!("Failed to create directory '{:?}': {}", dir, err);
                    }
                };

                let path = self.path.join(filename);
                match fs::write(&path, file) {
                    Ok(_) => {}
                    Err(err) => {
                        panic!("Failed to write file '{}': {}", filename, err);
                    }
                };
            }
        }
    }
}

