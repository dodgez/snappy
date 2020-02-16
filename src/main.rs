use clap::{App, AppSettings, Arg, SubCommand};
use std::path::Path;

mod checkout;
mod commit;
mod hash;
mod index;
mod log;
mod objects;
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
        .subcommand(
            SubCommand::with_name("commit")
                .about("Creates a snapshot of the staging area")
                .arg(
                    Arg::with_name("commit_message")
                        .help("A short description of the file changes")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("checkout")
                .about("Checkout a specific commit")
                .arg(
                    Arg::with_name("commit_hash")
                        .help("The hash of the commit to checkout")
                        .required(true),
                ),
        )
        .subcommand(SubCommand::with_name("log").about("Output the linear history of HEAD"))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("init") {
        repo::init(matches.is_present("force"));
    } else if let Some(matches) = matches.subcommand_matches("add") {
        stage::stage(&Path::new(matches.value_of("object_to_stage").unwrap()));
    } else if let Some(matches) = matches.subcommand_matches("commit") {
        commit::commit(matches.value_of("commit_message").unwrap());
    } else if let Some(matches) = matches.subcommand_matches("checkout") {
        checkout::checkout(matches.value_of("commit_hash").unwrap());
    } else if let Some(_matches) = matches.subcommand_matches("log") {
        log::log();
    }
}
