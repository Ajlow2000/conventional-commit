use std::{path::{Path, PathBuf}, sync::LazyLock};

use clap::{command, Arg, ArgAction, ArgMatches};

mod commit;
mod collect;

use collect::{collect_breaking_reason, collect_commit_scope, collect_commit_type, collect_description, collect_linked_ticket};
use commit::CommitMsg;
use serde::Deserialize;

static DEFAULT_CONFIG_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    PathBuf::from("./.conventional-commit.toml")
});

#[derive(Deserialize)]
struct Config {
    default_scopes: Option<Vec<String>>,
    default_messages: Option<Vec<String>>,
    ticket_identifier: Option<String>,
}

fn main() {
    // FIXME - fzf_wrapped spawns child processes that swallow this.
    //         Need to find a way to pass SIGINT to parent processes..
    ctrlc::set_handler(move || {
        println!("foo");
    }).expect("Error setting Ctrl-C handler");

    let args = parse_cli();

    let breaking_change = args.get_one::<bool>(SupportedArgIDs::BreakingChange.as_str()).unwrap_or_else(|| {
        panic!("An error occured while fetching {} value from parsed args.", SupportedArgIDs::BreakingChange.as_str());
    });
    let require_ticket = args.get_one::<bool>(SupportedArgIDs::RequireTicket.as_str()).unwrap_or_else(|| {
        panic!("An error occured while fetching {} value from parsed args.", SupportedArgIDs::RequireTicket.as_str());
    });
    let local_config_path = args.get_one::<PathBuf>(SupportedArgIDs::LocalConfigPath.as_str()).unwrap_or_else(|| {
        panic!("An error occured while fetching {} value from parsed args.", SupportedArgIDs::LocalConfigPath.as_str());
    });

    let mut default_scopes = vec![];
    let mut default_messages = vec![];
    if Path::exists(&local_config_path) {
        let config: Config = toml::from_str(
            &std::fs::read_to_string(&local_config_path)
                .expect(format!("Unable to read contents of {}", &local_config_path.to_str()
                    .expect("Could not get string representation of local_config_path"))
                    .as_str()
                )
        ).expect("Failed to deserialize file contents into expect Config struct");
        default_scopes = config.default_scopes.unwrap_or(vec![]);
        default_messages = config.default_messages.unwrap_or(vec![]);
    }



    let commit_type = collect_commit_type();
    let scope = collect_commit_scope(default_scopes);
    let desc = collect_description(default_messages);
    let breaking_reason = collect_breaking_reason(*breaking_change);
    let ticket = collect_linked_ticket(*require_ticket);

    let commit_msg = CommitMsg {
        commit_type: commit_type.clone(),
        scope: scope.clone(),
        desc: desc.clone(),
        breaking_reason: breaking_reason.clone(),
        related_ticket: ticket.clone(),
    };

    commit_msg.commit();
}

enum SupportedArgIDs {
    BreakingChange,
    LocalConfigPath,
    RequireTicket,
}

impl SupportedArgIDs {
    fn as_str(&self) -> &str {
        match *self {
            SupportedArgIDs::BreakingChange => "breaking-change",
            SupportedArgIDs::LocalConfigPath => "local-config",
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
        .arg(Arg::new(SupportedArgIDs::LocalConfigPath.as_str())
            .help("File in which to read the repo specific config TOML file.")
            .long(SupportedArgIDs::LocalConfigPath.as_str())
            .short('c')
            .value_name("local-config-path")
            .default_value(DEFAULT_CONFIG_PATH.to_str())
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
