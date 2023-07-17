use crate::process::encode::EncodeTarget;
use std::{fmt, fs, path::PathBuf};

pub struct Cache {
    path: PathBuf,
}

impl Cache {
    pub fn new(output_path: &PathBuf) -> Self {
        let path = output_path.join("cache");
        Self { path }
    }

    pub fn try_get(&self, svg: &str, format: &EncodeTarget, size: u32) -> Option<Vec<u8>> {
        let hash = md5::compute(format!("{}-{:?}-{}", svg, format, size));
        let hash = format!("{:x}", hash);

        let mut path = self.path.join(hash);
        path.set_extension(format.to_extension());

        match fs::read(path) {
            Ok(data) => Some(data),
            Err(_) => None,
        }
    }

    pub fn save(&self, svg: &str, format: &EncodeTarget, size: u32, raster: &Vec<u8>) {
        match fs::create_dir_all(&self.path) {
            Ok(_) => {}
            Err(err) => panic!("Failed to create cache directory: {}", err),
        }

        let hash = md5::compute(format!("{}-{:?}-{}", svg, format, size));
        let hash = format!("{:x}", hash);

        let mut path = self.path.join(hash);
        path.set_extension(format.to_extension());

        match fs::write(path, raster) {
            Ok(_) => {}
            Err(err) => panic!("Failed to write cache file: {}", err),
        }
    }
}

impl fmt::Debug for Cache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("<Cache {:?}>", self.path))
    }
}
