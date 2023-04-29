use std::{
    fs,
    path::PathBuf,
    collections::HashMap
};

pub struct Data {
    pub svg: HashMap<PathBuf, String>
}

fn walk_dir(path: &PathBuf, file_paths: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            walk_dir(&path, file_paths)?;
        } else {
            file_paths.push(path);
        }
    }
    Ok(())
}

impl Data {
    pub fn new() -> Self {
        Self {
            svg: HashMap::new(),
        }
    }

    pub fn load(&mut self) {
        let mut svg_paths: Vec<PathBuf> = Vec::new();

        let root_path = std::env::current_dir().unwrap().join("../assets/svg");
        walk_dir(&root_path, &mut svg_paths).unwrap();

        for path in svg_paths {
            let svg_file = fs::read_to_string(path.clone()).unwrap();
            self.svg.insert(path, svg_file);
        }
    }
}
