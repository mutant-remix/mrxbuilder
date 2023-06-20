mod load;
mod process;

use load::Pack;

use std::path::PathBuf;
fn main() {
    let mut pack = Pack::new();

    pack.load_all(&PathBuf::from("./sample-input/index.toml"));

    println!("{:#?}", pack);

    pack.build_all();
}
