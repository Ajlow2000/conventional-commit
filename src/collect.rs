use std::path::PathBuf;

use fzf_wrapped::{Border, Fzf, Layout};
use strum::IntoEnumIterator;

use crate::commit::SupportedType;

pub fn collect_commit_type() -> String {
    let supported_types: Vec<String> = SupportedType::iter()
        .map(|commit_type| {
            commit_type.to_commit_type().value
        })
        .collect();

    let mut fzf = Fzf::builder()
        .border_label(" Commit Type ")
        .border(Border::Rounded)
        .layout(Layout::Reverse)
        // .ansi(value)
        .custom_args(vec![
            "--height=~100%".to_string(),
        ])
        .build()
        .unwrap();
    fzf.run().expect("Failed to start fzf");
    fzf.add_items(supported_types).expect("Failed to add commit type entries into fzf");
    let users_selection = fzf.output().expect("Failed to get output from fzf selection").trim().to_string();
    users_selection
}

pub fn collect_commit_scope(mut default_scopes: Vec<String>) -> Option<String> {
    default_scopes.insert(0, "".to_string());
    let mut fzf = Fzf::builder()
        .border_label(" Commit Scope ")
        .border(Border::Rounded)
        .layout(Layout::Reverse)
        // .ansi(value)
        .custom_args(vec![
            "--height=~100%".to_string(),
            "--bind=enter:replace-query+print-query".to_string(),
        ])
        .build()
        .unwrap();
    fzf.run().expect("Failed to start fzf");
    fzf.add_items(default_scopes).expect("Failed to add default commit scope options into fzf");
    let users_selection = fzf.output().expect("Failed to get output from fzf selection").trim().to_string();

    if users_selection == "".to_string() {
        Some("".to_string())
    } else {
        Some(format!("({})", users_selection))
    }
}

pub fn collect_description(mut default_messages: Vec<String>) -> String {
    default_messages.insert(0, "".to_string());
    let mut fzf = Fzf::builder()
        .border_label(" Commit Description")
        .border(Border::Rounded)
        .layout(Layout::Reverse)
        // .ansi(value)
        .custom_args(vec![
            "--height=~100%".to_string(),
            "--bind=enter:replace-query+print-query".to_string(),
        ])
        .build()
        .unwrap();
    fzf.run().expect("Failed to start fzf");
    fzf.add_items(default_messages).expect("Failed to add commit scope options into fzf");
    let users_selection = fzf.output().expect("Failed to get output from fzf selection").trim().to_string();

    let desc = match users_selection.as_str() {
        s if s.is_empty() => collect_text_from_vipe("".to_string()),
        s if s.ends_with("::" ) => {
            let mut cleaned_text = s.to_owned();
            cleaned_text.truncate(s.len() - 2); // this doesnt feel right. runtime panic that cant be coded around?
            collect_text_from_vipe(cleaned_text)
        },
        _ => users_selection,
    };

    desc
}

fn collect_text_from_vipe(existing_text: String)  -> String {
    let text = subprocess::Exec::shell("vipe").stdin(existing_text.as_str()).capture().expect("No valid output captured from vipe").stdout_str();
    text
}

pub fn collect_breaking_reason(collect: bool) -> Option<String> {
    if collect {
        let reasons = vec![""];

        let mut fzf = Fzf::builder()
            .border_label(" Breaking Change: Reason")
            .border(Border::Rounded)
            .layout(Layout::Reverse)
            // .ansi(value)
            .custom_args(vec![
                "--height=~100%".to_string(),
                "--bind=enter:replace-query+print-query".to_string(),
            ])
            .build()
            .unwrap();
        fzf.run().expect("Failed to start fzf");
        fzf.add_items(reasons).expect("Failed to add default breaking reasons into fzf");
        let users_selection = fzf.output().expect("Failed to get output from fzf selection").trim().to_string();

        Some(format!("BREAKING REASON: {}", users_selection))
    } else {
        None
    }
}

pub fn collect_linked_ticket(collect: bool) -> Option<String> {
    if collect {
        let options = vec![""];

        let mut fzf = Fzf::builder()
            .border_label(" Linked Ticket ")
            .border(Border::Rounded)
            .layout(Layout::Reverse)
            // .ansi(value)
            .custom_args(vec![
                "--height=~100%".to_string(),
                "--bind=enter:replace-query+print-query".to_string(),
            ])
            .build()
            .unwrap();
        fzf.run().expect("Failed to start fzf");
        fzf.add_items(options).expect("Failed to add default options into fzf");
        let users_selection = fzf.output().expect("Failed to get output from fzf selection").trim().to_string();

        Some(format!("TICKET: {}", users_selection))
    } else {
        None
    }
}
