use std::path::{Path, PathBuf};

use clap::{command, Arg, ArgAction, ArgMatches};

fn main() {
    let matches = parse_cli();
    println!("Hello, world!");
    if let Some(breaking_change) = matches.get_one::<bool>("breaking_change") {
        println!("{}", breaking_change);
    }
}

fn parse_cli() -> ArgMatches {
    let matches = command!()
        .arg(Arg::new("breaking_change")
            .help("Indicates if the commit includes version breaking changes.")
            .long("breaking-change")
            .alias("breaking")
            .short('b')
            .action(ArgAction::SetTrue)
        )
        .arg(Arg::new("commit-scopes")
            .help("File in which to read commit scopes from. Defaults to './.commit-scopes' if file exists.")
            .long_help("Values in this file are expected to be delimited by newlines. Entries should be ")
            .long("commit-scopes")
            .short('s')
            .value_name("commit-scopes-file-path")
            .value_parser(clap::value_parser!(PathBuf))
        )
        .get_matches();
    matches
}
