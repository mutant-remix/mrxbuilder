use crate::load::manifest::{Container, TarCompression};
use bzip2::{write::BzEncoder, Compression as BzipCompression};
use std::{
    fs::{self, File},
    io::{ErrorKind::NotFound, Write},
    path::PathBuf,
};
use tar::{Builder as TarBuilder, Header as TarHeader};
use xz2::write::XzEncoder;
use zip::{write::FileOptions, CompressionMethod as ZipCompressionMethod, ZipWriter};
use zstd::stream::write::Encoder as ZstdEncoder;
use libflate::gzip::Encoder as GzipEncoder;

enum TarCompressor<'a> {
    None(TarBuilder<File>),
    Gzip(TarBuilder<GzipEncoder<File>>),
    Bzip2(TarBuilder<BzEncoder<File>>),
    Xz(TarBuilder<XzEncoder<File>>),
    Zstd(TarBuilder<ZstdEncoder<'a, File>>),
}

enum PackageKind<'a> {
    Dry,
    Directory,
    Zip(ZipWriter<File>, ZipCompressionMethod),
    Tar(TarCompressor<'a>),
}
pub struct Package<'a> {
    kind: PackageKind<'a>,
    path: PathBuf,
}

impl Package<'_> {
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
                    let extension = match compression {
                        ZipCompressionMethod::Stored => "zip",
                        ZipCompressionMethod::Deflated => "zip",
                        ZipCompressionMethod::Bzip2 => "bz2.zip",
                        ZipCompressionMethod::Zstd => "zst.zip",
                        _ => panic!("Unsupported compression method"),
                    };

                    let file = match File::create(path.with_extension(extension)) {
                        Ok(file) => file,
                        Err(err) => {
                            panic!(
                                "Failed to create zip file '{:?}.{}': {}",
                                path, extension, err
                            );
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
                            panic!(
                                "Failed to create tar file '{:?}.{}': {}",
                                path, extension, err
                            );
                        }
                    };

                    match compression {
                        TarCompression::None => {
                            let writer = TarBuilder::new(file);
                            PackageKind::Tar(TarCompressor::None(writer))
                        }
                        TarCompression::Gzip => {
                            let writer = TarBuilder::new(GzipEncoder::new(file).unwrap());
                            PackageKind::Tar(TarCompressor::Gzip(writer))
                        }
                        TarCompression::Bzip2 => {
                            let writer =
                                TarBuilder::new(BzEncoder::new(file, BzipCompression::best()));
                            PackageKind::Tar(TarCompressor::Bzip2(writer))
                        }
                        TarCompression::Xz => {
                            let writer = TarBuilder::new(XzEncoder::new(file, 9));
                            PackageKind::Tar(TarCompressor::Xz(writer))
                        }
                        TarCompression::Zstd => {
                            let writer = TarBuilder::new(ZstdEncoder::new(file, 21).unwrap());
                            PackageKind::Tar(TarCompressor::Zstd(writer))
                        }
                    }
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
                let options = FileOptions::default().compression_method(*compression);

                match writer.start_file(filename, options) {
                    Ok(_) => {}
                    Err(err) => {
                        panic!(
                            "Failed to start file '{}' in zip '{:?}.zip': {}",
                            filename, self.path, err
                        );
                    }
                };

                match writer.write(file) {
                    Ok(_) => {}
                    Err(err) => {
                        panic!(
                            "Failed to write file '{}' to zip '{:?}.zip': {}",
                            filename, self.path, err
                        );
                    }
                };
            }
            PackageKind::Tar(writer) => {
                let mut header = TarHeader::new_gnu();

                header.set_path(filename).unwrap();
                header.set_size(file.len() as u64);
                header.set_mode(0o644);
                header.set_cksum();

                let result = match writer {
                    TarCompressor::None(writer) => {
                        writer.append_data(&mut header, filename, file.as_slice())
                    }
                    TarCompressor::Gzip(writer) => {
                        writer.append_data(&mut header, filename, file.as_slice())
                    }
                    TarCompressor::Bzip2(writer) => {
                        writer.append_data(&mut header, filename, file.as_slice())
                    }
                    TarCompressor::Xz(writer) => {
                        writer.append_data(&mut header, filename, file.as_slice())
                    }
                    TarCompressor::Zstd(writer) => {
                        writer.append_data(&mut header, filename, file.as_slice())
                    }
                };

                match result {
                    Ok(_) => {}
                    Err(err) => {
                        panic!(
                            "Failed to write file '{}' to tar '{:?}.tar': {}",
                            filename, self.path, err
                        );
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

    pub fn finish(self) {

        match self.kind {
            PackageKind::Dry => {}
            PackageKind::Zip(mut writer, _) => {
                match writer.finish() {
                    Ok(_) => {}
                    Err(err) => {
                        panic!("Failed to close zip file '{:?}': {}", self.path, err);
                    }
                };
            }
            PackageKind::Tar(writer) => {
                let writer = writer;

                let result = match writer {
                    TarCompressor::None(mut writer) => {
                        writer.finish()
                    },
                    TarCompressor::Gzip(writer) => {
                        writer.into_inner().unwrap().finish().unwrap();
                        Ok(())
                    },
                    TarCompressor::Bzip2(writer) => {
                        writer.into_inner().unwrap().finish().unwrap();
                        Ok(())
                    },
                    TarCompressor::Xz(writer) => {
                        writer.into_inner().unwrap().finish().unwrap();
                        Ok(())
                    },
                    TarCompressor::Zstd(writer) => {
                        writer.into_inner().unwrap().finish().unwrap();
                        Ok(())
                    }
                };

                match result {
                    Ok(_) => {}
                    Err(err) => {
                        panic!("Failed to close tar file '{:?}': {}", self.path, err);
                    }
                };
            }
            PackageKind::Directory => {}
        }
    }
}
