mod load;
mod rasterize;
mod encode;

use load::Data;

use std::path::PathBuf;

fn main() {
    let mut data = Data::new();
    data.load(&PathBuf::from("./sample-input/index.toml"));
}
