use std::path::PathBuf;

use clap::{command, Arg, ArgAction, ArgMatches};

mod commit;
mod collect;
use commit::CommitMsg;

fn main() {
    let _ = parse_cli();
    println!("Hello, world!");
    // if let Some(breaking_change) = matches.get_one::<bool>("breaking_change") {
    //     println!("{}", breaking_change);
    // }



    // let mut fzf = Fzf::default();
    // let mut fzf = Fzf::builder()
    //     .layout(Layout::Reverse)
    //     .border(Border::Rounded)
    //     .border_label(" Commit Type ")
    //     .build()
    //     .unwrap();
    //
    // fzf.run().expect("Failed to start fzf");
    //
    // // fzf.add_items(colours).expect("Failed to add items");
    //
    // let users_selection = fzf.output().expect("Failed to get the user's output");
    // println!("Selected: {}", users_selection);
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
            .help("File in which to read commit scopes from. Defaults to './.commit-scopes' if file exists. Values in this file are expected to be delimited by newlines.")
            .long("commit-scopes")
            .short('s')
            .value_name("commit-scopes-file-path")
            .value_parser(clap::value_parser!(PathBuf))
        )
        .arg(Arg::new("require-ticket")
            .help("Indicates if the commit should include a footer linking the related ticket.")
            .long("require-ticket")
            .short('t')
            .action(ArgAction::SetTrue)
        )
        .get_matches();
    matches
}
