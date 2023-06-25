use std::path::PathBuf;

mod load;
use load::Pack;

mod logger;
use logger::Logger;

mod process;

fn main() {
    let mut logger = Logger::init();
    logger.info(&format!("Using {} CPUs", num_cpus::get() as u64));

    let mut pack = Pack::new(logger);
    pack.load_all(&PathBuf::from("./sample-input/index.toml"));

    // println!("{:#?}", pack);

    pack.build_all();
}
