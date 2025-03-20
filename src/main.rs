use std::{path::{Path, PathBuf}, sync::LazyLock};

use clap::{command, Arg, ArgAction, ArgMatches};

mod commit;
mod collect;

use collect::{collect_breaking_reason, collect_commit_scope, collect_commit_type, collect_description, collect_linked_ticket};
use commit::CommitMsg;

static DEFAULT_COMMIT_SCOPES_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    PathBuf::from("./.commit-scopes")
});

fn main() {
    let args = parse_cli();

    let breaking_change = args.get_one::<bool>(SupportedArgIDs::BreakingChange.as_str()).unwrap_or_else(|| {
        panic!("An error occured while fetching {} value from parsed args.", SupportedArgIDs::BreakingChange.as_str());
    });
    let require_ticket = args.get_one::<bool>(SupportedArgIDs::RequireTicket.as_str()).unwrap_or_else(|| {
        panic!("An error occured while fetching {} value from parsed args.", SupportedArgIDs::RequireTicket.as_str());
    });
    let commit_scopes_path = args.get_one::<PathBuf>(SupportedArgIDs::CommitScopesPath.as_str()).unwrap_or_else(|| {
        panic!("An error occured while fetching {} value from parsed args.", SupportedArgIDs::CommitScopesPath.as_str());
    });

    let commit_type = collect_commit_type();
    let scope = collect_commit_scope(&commit_scopes_path);
    let desc = collect_description();
    let breaking_reason = collect_breaking_reason(*breaking_change);
    let ticket = collect_linked_ticket(*require_ticket);

    let commit_msg = CommitMsg {
        commit_type: commit_type.clone(),
        scope: scope.clone(),
        desc: desc.clone(),
        breaking_reason: breaking_reason.clone(),
        related_ticket: ticket.clone(),
    };

    println!("{}", commit_msg.commit());

    // println!("Type: {}", &commit_type);
    // println!("Scope: {}", &scope.unwrap_or("none".to_string()));
    // println!("desc: {}", &desc);
    // println!("breaking: {}", &breaking_reason.unwrap_or("none".to_string()));
    // println!("ticket: {}", &ticket.unwrap_or("none".to_string()));
}

enum SupportedArgIDs {
    BreakingChange,
    CommitScopesPath,
    RequireTicket,
}

impl SupportedArgIDs {
    fn as_str(&self) -> &str {
        match *self {
            SupportedArgIDs::BreakingChange => "breaking-change",
            SupportedArgIDs::CommitScopesPath => "commit-scopes-path",
            SupportedArgIDs::RequireTicket => "require-ticket",
        }
    }
}


fn parse_cli() -> ArgMatches {
    command!()
        .arg(Arg::new(SupportedArgIDs::BreakingChange.as_str())
            .help("Indicates if the commit includes version breaking changes.")
            .long(SupportedArgIDs::BreakingChange.as_str())
            .visible_alias("breaking")
            .short('b')
            .action(ArgAction::SetTrue)
        )
        .arg(Arg::new(SupportedArgIDs::CommitScopesPath.as_str())
            .help("File in which to read commit scopes from. Values in this file are expected to be delimited by newlines.")
            .long(SupportedArgIDs::CommitScopesPath.as_str())
            .visible_alias("scopes")
            .short('s')
            .value_name("commit-scopes-file-path")
            .default_value(DEFAULT_COMMIT_SCOPES_PATH.to_str())
            .value_parser(clap::builder::PathBufValueParser::new())
        )
        .arg(Arg::new(SupportedArgIDs::RequireTicket.as_str())
            .help("Indicates if the commit should include a footer linking the related ticket.")
            .long(SupportedArgIDs::RequireTicket.as_str())
            .visible_alias("ticket")
            .short('t')
            .action(ArgAction::SetTrue)
        )
        .get_matches()
}
