use clap::{App, Arg, AppSettings, SubCommand};

mod repo;

fn main() {
    let matches = App::new("Snappy")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("A distributed version control system")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(SubCommand::with_name("init")
            .about("Creates an empty snappy repository")
            .arg(Arg::with_name("force")
                .help("Overwrites an existing repository")
                .short("f")))
        .get_matches();
    
    if let Some(matches) = matches.subcommand_matches("init") {
        repo::init(matches.is_present("force"));
    }
}
