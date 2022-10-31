use clap::{Arg, ArgAction, Command};

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

    if let Some(file) = matches.get_one::<String>("file") {
        println!("begin decompile file:{}", file);
    }
}
