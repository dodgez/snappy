use clap::{App, AppSettings, Arg, SubCommand};
use std::path::Path;

mod repo;
mod stage;

fn main() {
    let matches = App::new("Snappy")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("A distributed version control system")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("init")
                .about("Creates an empty snappy repository")
                .arg(
                    Arg::with_name("force")
                        .help("Overwrites an existing repository")
                        .short("f"),
                ),
        )
        .subcommand(
            SubCommand::with_name("add")
                .about("Adds an object to the staging area")
                .arg(
                    Arg::with_name("object_to_stage")
                        .help("The object to add to staging")
                        .required(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("init") {
        repo::init(matches.is_present("force"));
    }

    if let Some(matches) = matches.subcommand_matches("add") {
        stage::stage(&Path::new(matches.value_of("object_to_stage").unwrap()));
    }
}
