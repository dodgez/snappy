use clap::{App, AppSettings, Arg, SubCommand};
use std::path::Path;
use whoami::username;

use snappy::{branch, checkout, commit, log, repo, stage};

fn main() {
    let name = username();

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
                        .short("m")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("author_name")
                        .help("The name of the author")
                        .short("a")
                        .takes_value(true)
                        .default_value(&name),
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
        .subcommand(
            SubCommand::with_name("branch")
                .about("Create a new branch")
                .arg(
                    Arg::with_name("branch_name")
                        .help("The name of the new branch")
                        .required(true),
                ),
        )
        .subcommand(SubCommand::with_name("log").about("Output the linear history of HEAD"))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("init") {
        repo::init(matches.is_present("force"));
    } else if let Some(matches) = matches.subcommand_matches("add") {
        let object = matches.value_of("object_to_stage").unwrap();
        match stage::stage(&Path::new(object)) {
            Ok(_) => (),
            Err(e) => panic!(e),
        }
    } else if let Some(matches) = matches.subcommand_matches("commit") {
        let message = matches.value_of("commit_message").unwrap();
        let author = matches.value_of("author_name").unwrap();
        match commit::commit(message, author) {
            Ok(hash) => println!("{}", hash),
            Err(e) => panic!(e),
        }
    } else if let Some(matches) = matches.subcommand_matches("checkout") {
        match checkout::checkout(matches.value_of("commit_hash").unwrap()) {
            Ok(_) => (),
            Err(e) => panic!(e),
        }
    } else if let Some(_matches) = matches.subcommand_matches("log") {
        match log::log() {
            Ok(_) => (),
            Err(e) => panic!(e),
        }
    } else if let Some(matches) = matches.subcommand_matches("branch") {
        match branch::branch(matches.value_of("branch_name").unwrap()) {
            Ok(_) => (),
            Err(e) => panic!(e),
        }
    }
}
