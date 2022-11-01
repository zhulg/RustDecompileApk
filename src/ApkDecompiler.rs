use std::{env, path::PathBuf};

pub struct Decompiler {
    apk_path: PathBuf,
    current_dir: PathBuf,
    output_path: PathBuf,
    exe_dir: PathBuf,
}

impl Decompiler {
    pub fn new(apk_path: PathBuf) -> Self {
        let current_dir = env::current_dir().unwrap();
        println!("current_dir={}", current_dir.display());
        let mut exe_dir = env::current_exe().unwrap();
        println!("exe_dir={}", exe_dir.display());
        exe_dir.pop();
        let output_path = current_dir.join("output");
        Self {
            apk_path,
            current_dir,
            exe_dir,
            output_path,
        }
    }
}
