use std::{
    env,
    fs::{self},
    io::{Error, ErrorKind, Result},
    path::PathBuf,
    process::Command,
};

use console::style;
use execute::Execute;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use text2art::{BasicFonts, Font, Printer};

pub struct Decompiler {
    apk_path: PathBuf,
    output_path: PathBuf,
    exe_dir: PathBuf,
}

impl Decompiler {
    pub fn new(apk_path: PathBuf) -> Self {
        let current_dir = env::current_dir().unwrap();
        let mut exe_dir = env::current_exe().unwrap();
        exe_dir.pop();

        let output_path = current_dir.join("output");
        Self {
            apk_path,
            exe_dir,
            output_path,
        }
    }

    pub fn start_decompile(&self) -> Result<()> {
        self.show_tools_info()?;
        self.create_output_dir()?;
        self.start_dex2jar()?;
        self.start_decompile_class()?;
        self.start_decompile_res()?;
        self.open_output()?;

        Ok(())
    }

    ///use dex2jar get APK's jar in output_path
    pub fn start_dex2jar(&self) -> Result<()> {
        let mut command = Command::new("sh");

        command
            .arg(self.exe_dir.join("lib/dex2jar/d2j-dex2jar.sh"))
            .arg("-f")
            .arg(&self.apk_path)
            .arg("-o")
            .arg(self.output_path.join("app.jar"));

        execute_state(command, "dex2jar");
        Ok(())
    }

    /// use jd-cli decompile jar to class
    /// TODO: some apk decompile error...
    pub fn start_decompile_class(&self) -> Result<()> {
        let jar_file = self.output_path.join("app.jar");
        let mut command = Command::new("sh");
        command
            .arg(self.exe_dir.join("lib/jd-cli/jd-cli"))
            .arg("-od")
            .arg(&self.output_path.join("classes"))
            .arg(&jar_file);
        execute_state(command, "decompile class");
        fs::remove_file(jar_file)?;
        Ok(())
    }

    /// use apktool decompile resources
    pub fn start_decompile_res(&self) -> Result<()> {
        let mut command = Command::new("sh");
        command
            .arg(self.exe_dir.join("lib/apktool/apktool"))
            .arg("d")
            .arg(&self.apk_path)
            .arg("-o")
            .arg(self.output_path.join("Resource"));

        execute_state(command, "decompile Resource");
        Ok(())
    }

    //// create output dir
    pub fn create_output_dir(&self) -> Result<()> {
        if self.output_path.exists() {
            println!(
                "{}",
                style(format!(
                    "begin del old file...in {}",
                    &self.output_path.display()
                ))
                .yellow()
            );

            fs::remove_dir_all(&self.output_path)?;
        }
        fs::create_dir(&self.output_path)?;
        println!(
            "{}",
            style(format!(
                "\u{2705} create ouput:{}",
                &self.output_path.display()
            ))
            .green()
        );
        Ok(())
    }

    /// check apk is exists
    pub fn check_apk_path(&self) -> Result<()> {
        if self.apk_path.exists()
            && self.apk_path.extension().is_some()
            && self.apk_path.extension().unwrap().eq("apk")
        {
            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::NotFound,
                "check your apk path or file is exists, use: ApkDecompiler -f xxxx.apk or ApkDecompiler xxxx.apk",
            ))
        }
    }

    ///
    pub fn show_tools_info(&self) -> Result<()> {
        let font = match Font::from_basic(BasicFonts::Big) {
            Ok(font) => font,
            Err(_) => panic!("something wrong with font"),
        };
        let prntr = Printer::with_font(font);
        prntr.print_to_stdio("Spark Decompiler").ok();
        Ok(())
    }

    /// open dir
    pub fn open_output(&self) -> Result<()> {
        Command::new("open").arg(&self.output_path).output()?;
        Ok(())
    }
}

/// execute state show
fn execute_state(mut command: Command, command_name: &str) {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(125));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} {msg}")
            .unwrap()
            .tick_strings(&["∙∙∙", "●∙∙", "∙●∙", "∙∙●", "∙∙∙"]),
    );
    pb.set_message(format!("do {}...", command_name.to_string()));
    if let Some(exit_code) = command.execute().unwrap() {
        if exit_code == 0 {
            pb.finish_and_clear();
            // https://www.compart.com/en/unicode/U+2705
            println!(
                "{}",
                style(format!("\u{2705} {}...done", command_name)).green()
            );
        } else {
            pb.finish_and_clear();
            eprintln!(
                "{}",
                style(format!("\u{2757} {}...failed", command_name)).red()
            );
        }
    } else {
        pb.finish_and_clear();
        eprintln!("{}{}", command_name, style("..Interrupted!").yellow());
    }
}
