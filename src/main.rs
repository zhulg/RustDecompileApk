mod ApkDecompiler;
use crate::ApkDecompiler::Decompiler;
use clap::{Arg, ArgAction, Command};
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
    println!("begin decompile file:{}", file_path);
    let apk_path = PathBuf::from(file_path);
    let apk_decompiler = Decompiler::new(apk_path);
    apk_decompiler.create_output_dir();
    apk_decompiler.start_dex2jar();
    apk_decompiler.start_decompile_class();
    apk_decompiler.start_decompile_res();
    apk_decompiler.open_output();
}
