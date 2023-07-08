use crate::load::manifest::{Container, TarCompression};
use std::{
    fs::{self, File},
    io::{ErrorKind::NotFound, Write},
    path::PathBuf,
};
use zip::{write::FileOptions, CompressionMethod, ZipWriter};

enum PackageKind {
    Dry,
    Directory,
    Zip(ZipWriter<File>, CompressionMethod),
    Tar(File, TarCompression),
}

pub struct Package {
    kind: PackageKind,
    path: PathBuf,
}

impl Package {
    pub fn new(kind: &Container, path: &PathBuf, dry: bool) -> Self {
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

        let kind = if dry {
            PackageKind::Dry
        } else {
            match kind {
                Container::Zip(compression) => {
                    let file = match File::create(path.with_extension("zip")) {
                        Ok(file) => file,
                        Err(err) => {
                            panic!("Failed to create zip file '{:?}.zip': {}", path, err);
                        }
                    };

                    PackageKind::Zip(ZipWriter::new(file), *compression)
                }
                Container::Tar(compression) => {
                    let extension = match compression {
                        TarCompression::None => "tar",
                        TarCompression::Gzip => "tar.gz",
                        TarCompression::Bzip2 => "tar.bz2",
                        TarCompression::Xz => "tar.xz",
                        TarCompression::Zstd => "tar.zst",
                    };

                    let file = match File::create(path.with_extension(extension)) {
                        Ok(file) => file,
                        Err(err) => {
                            panic!("Failed to create tar file '{:?}.tar': {}", path, err);
                        }
                    };

                    PackageKind::Tar(file, compression.clone())
                }
                Container::Directory => PackageKind::Directory,
            }
        };

        Self {
            kind,
            path: path.clone(),
        }
    }

    pub fn add_file(&mut self, file: &Vec<u8>, filename: &str) {
        match &mut self.kind {
            PackageKind::Dry => {}
            PackageKind::Zip(writer, compression) => {
                let options =
                    FileOptions::default().compression_method(*compression);

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
            PackageKind::Tar(file, compression) => {
                unimplemented!("Tar containers are not yet supported")
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
            PackageKind::Dry => {}
            PackageKind::Zip(writer, _) => {
                match writer.finish() {
                    Ok(_) => {}
                    Err(err) => {
                        panic!("Failed to close zip file '{:?}': {}", self.path, err);
                    }
                };
            }
            PackageKind::Tar(writer, compression) => {
                unimplemented!("Tar containers are not yet supported")
            }
            PackageKind::Directory => {}
        }
    }
}
