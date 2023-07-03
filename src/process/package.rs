use crate::load::manifest::Container;
use std::{
    fs::{self, File},
    io::{ErrorKind::NotFound, Write},
    path::PathBuf,
};
use zip::{write::FileOptions, CompressionMethod, ZipWriter};

enum PackageKind {
    Directory,
    Zip(ZipWriter<File>),
}

pub struct Package {
    kind: PackageKind,
    path: PathBuf,
}

impl Package {
    pub fn new(kind: &Container, path: &PathBuf) -> Self {
        match fs::remove_dir_all(path) {
            Ok(_) => {}
            Err(err) => {
                if err.kind() != NotFound {
                    panic!(
                        "Failed to remove old directory '{}': {}",
                        path.display(),
                        err
                    );
                }
            }
        }

        Self {
            kind: match kind {
                Container::Zip => {
                    let file = match File::create(path.with_extension("zip")) {
                        Ok(file) => file,
                        Err(err) => {
                            panic!("Failed to create zip file '{:?}.zip': {}", path, err);
                        }
                    };

                    PackageKind::Zip(ZipWriter::new(file))
                }
                Container::Directory => PackageKind::Directory,
            },
            path: path.clone(),
        }
    }

    pub fn add_file(&mut self, file: &Vec<u8>, filename: &str) {
        match &mut self.kind {
            PackageKind::Zip(writer) => {
                let options =
                    FileOptions::default().compression_method(CompressionMethod::Deflated);

                match writer.start_file(filename, options) {
                    Ok(_) => {}
                    Err(err) => {
                        panic!("Failed to start file '{}' in zip '{:?}.zip': {}", filename, self.path, err);
                    }
                };

                match writer.write(file) {
                    Ok(_) => {}
                    Err(err) => {
                        panic!("Failed to write file '{}' to zip '{:?}.zip': {}", filename, self.path, err);
                    }
                };
            }
            PackageKind::Directory => {
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

    pub fn finish(&mut self) {
        match &mut self.kind {
            PackageKind::Zip(writer) => {
                match writer.finish() {
                    Ok(_) => {}
                    Err(err) => {
                        panic!("Failed to close zip file '{:?}': {}", self.path, err);
                    }
                };
            }
            PackageKind::Directory => {}
        }
    }
}
