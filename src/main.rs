mod ApkDecompiler;
use crate::ApkDecompiler::Decompiler;
use clap::{Arg, ArgAction, Command};
use console::style;
use std::path::PathBuf;
fn main() {
    let matches = Command::new("Decompile APK")
        .author("lg.json@gmail.com")
        .version("1.0.0")
        .about("Decompile APK for Android")
        .arg(
            Arg::new("file")
                .action(ArgAction::Set)
                .index(1)
                .default_value("-")
                .help("The path to your apk."),
        )
        .after_help(
            "Longer explanation to appear after the options when \
                  displaying the help information from --help or -h",
        )
        .get_matches();

    let file_path = match matches.get_one::<String>("file") {
        Some(it) => it,
        _ => return,
    };
    let apk_path = PathBuf::from(file_path);
    let apk_decompiler = Decompiler::new(apk_path);

    if let Err(e) = apk_decompiler.check_apk_path() {
        eprintln!("{}", style(format!("  Error: {}", e.to_string())).red());
    } else {
        apk_decompiler.start_decompile().unwrap();
    }
}
