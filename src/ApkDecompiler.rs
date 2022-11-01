use std::{
    env,
    fs::{self, create_dir},
    io::Result,
    path::PathBuf,
    process::Command,
    sync::Arc,
};

use clap::Arg;

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

    ///use dex2jar get APK's jar in output_path
    pub fn start_dex2jar(&self) -> Result<()> {
        println!("begin dex2jar...");
        // println!("begin decompile apkpath:{}", self.apk_path.display());
        // println!("begin decompile output_path:{}", self.output_path.display());
        // println!("begin decompile exe_dir_path:{}", self.exe_dir.display());
        Command::new("sh")
            .arg(self.exe_dir.join("lib/dex2jar/d2j-dex2jar.sh"))
            .arg("-f")
            .arg(&self.apk_path)
            .arg("-o")
            .arg(self.output_path.join("app.jar"))
            .output()?;
        println!("dex2jar...done");
        Ok(())
    }

    ///Users/zhulianggang/Downloads/jd-cli-jd-cli-1.2.1/jd-cli/src/main/bin/jd-cli  -od ./decompiled  /Users/zhulianggang/test/test/test/app.jar
    pub fn start_decompile_class(&self) -> Result<()> {
        println!("begin decompile class...");
        let jar_file = self.output_path.join("app.jar");

        Command::new("sh")
            .arg(self.exe_dir.join("lib/jd-cli/jd-cli"))
            .arg("-od")
            .arg(&self.output_path.join("classes"))
            .arg(&jar_file)
            .output()?;
        println!("decompile class...done");
        fs::remove_file(jar_file)?;
        Ok(())
    }

    pub fn create_output_dir(&self) -> Result<()> {
        if self.output_path.exists() {
            fs::remove_dir_all(&self.output_path)?;
        }
        fs::create_dir(&self.output_path)?;
        println!("create output:={}", &self.output_path.display());
        Ok(())
    }
}
