use std::{env, fs, path::PathBuf};

mod pack;
use pack::Pack;

mod load;

mod logger;
use logger::Logger;

mod process;

fn main() {
    let mut logger = Logger::init();

    logger.register_panic_hook();
    logger.set_stage_count(1);

    logger.info(&format!("Using {} CPUs", num_cpus::get()));

    let mut args = env::args();
    args.nth(0);

    let manifest_path = match args.nth(0) {
        Some(path) => PathBuf::from(path),
        None => {
            panic!("Missing input manifest path. Usage: <input manifest> <output directory> <tag1,tag2>");
        }
    };
    let output_path = match args.nth(0) {
        Some(path) => PathBuf::from(path),
        None => {
            panic!("Missing output directory path. Usage: <input manifest> <output directory> <tag1,tag2>");
        }
    };
    let tags = match args.nth(0) {
        Some(tags) => tags.split(',').map(|s| s.to_string()).collect::<Vec<_>>(),
        None => {
            panic!("Missing tags. Usage: <input manifest> <output directory> <tag1,tag2>");
        }
    };

    let dry = match args.nth(0) {
        Some(dry) => {
            if dry == "--dry" {
                logger.info("Running in dry run mode. No files will be written.");
                true
            } else {
                panic!("Unknown argument: {}", dry);
            }
        }
        None => {
            match fs::create_dir_all(&output_path) {
                Ok(_) => {}
                Err(err) => panic!("Failed to create output directory: {}", err),
            };
            false
        }
    };

    let mut pack = Pack::new(logger, output_path);

    pack.load_all(&manifest_path);
    pack.build_tags(tags, dry);

    if let Some(save_thread) = pack.save_thread.take() {
        save_thread.join().unwrap();
    }

    pack.logger.finish()
}
