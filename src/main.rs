use clap::{App, AppSettings, Arg, SubCommand};
use std::io;
use std::path::Path;
use whoami;

use snappy::{branch, checkout, commit, diff, log, merge, repo, stage};

fn main() -> Result<(), io::Error> {
    let name = whoami::username();

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
        .subcommand(
            SubCommand::with_name("diff")
                .about("Get differences between the working copy and history copy of a file")
                .arg(
                    Arg::with_name("file_to_diff")
                        .help("The file to diff")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("merge")
                .about("Merge another branch into current")
                .arg(
                    Arg::with_name("object_name")
                        .help("The name of the object to merge")
                        .required(true),
                ),
        )
        .subcommand(SubCommand::with_name("log").about("Output the linear history of HEAD"))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("init") {
        repo::init(matches.is_present("force"))?;
    } else if let Some(matches) = matches.subcommand_matches("add") {
        let object = matches.value_of("object_to_stage").unwrap();
        stage::stage(&Path::new(object))?;
    } else if let Some(matches) = matches.subcommand_matches("commit") {
        let message = matches.value_of("commit_message").unwrap();
        let author = matches.value_of("author_name").unwrap();
        println!("{}", commit::commit(message, author)?);
    } else if let Some(matches) = matches.subcommand_matches("checkout") {
        checkout::checkout(matches.value_of("commit_hash").unwrap())?;
    } else if let Some(_matches) = matches.subcommand_matches("log") {
        log::log()?;
    } else if let Some(matches) = matches.subcommand_matches("branch") {
        branch::branch(matches.value_of("branch_name").unwrap())?;
    } else if let Some(matches) = matches.subcommand_matches("merge") {
        merge::merge(matches.value_of("object_name").unwrap())?;
    } else if let Some(matches) = matches.subcommand_matches("diff") {
        diff::diff(matches.value_of("file_to_diff").unwrap())?;
    }

    Ok(())
}
