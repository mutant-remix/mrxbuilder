use std::path::PathBuf;

mod load;
use load::Pack;

mod logger;
use logger::Logger;

mod process;

fn main() {
    let mut logger = Logger::init();

    logger.register_panic_hook();
    logger.set_stage_count(1);

    logger.info(&format!("Using {} CPUs", num_cpus::get() as u64));

    let mut pack = Pack::new(logger);

    pack.load_all(&PathBuf::from("./sample-input/index.toml"));
    pack.build_all();

    pack.logger.finish()
}
