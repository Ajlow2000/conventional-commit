#[derive(Debug)]
pub struct CommitMsg {
    commit_type: SupportedType,
    scope: Option<String>,
    title: String,
    body: Option<String>,
    related_ticket: Option<Ticket>,
    breaking_reason: Option<String>,
}

impl CommitMsg {
    pub fn new() {
        unimplemented!()
    }

    pub fn commit() {
        unimplemented!()
    }

    fn build(self) {
        unimplemented!()
    }
}

#[derive(Debug)]
struct CommitType {
    value: String,
    description: String,
}
impl CommitType {
    fn new(value: &str, description: &str) -> CommitType {
        CommitType {
            value: value.to_string(), 
            description: description.to_string(),
        }
    }
}

#[derive(Debug)]
enum SupportedType {
    Feature,
    Fix,
    Build,
    Chore,
    CI,
    Docs,
    Refactor,
    Style,
    Test,
    Wip,
}

impl SupportedType {
    fn value(&self) -> CommitType {
        match *self {
            SupportedType::Feature => CommitType::new("feat", "Implementation of a new feature"),
            SupportedType::Fix => CommitType::new("fix", "A bug fix"),
            SupportedType::Build => CommitType::new("build", "Changes to a build system"),
            SupportedType::Chore => CommitType::new("chore", "Updating grunt tasks etc. No production code change"),
            SupportedType::CI => CommitType::new("ci", "Changes to CI/CD pipeline"),
            SupportedType::Docs => CommitType::new("docs", "Documentation only change"),
            SupportedType::Refactor => CommitType::new("refactor", "A code change that neither fixes a bug nor adds a feature"),
            SupportedType::Style => CommitType::new("style", "Changes that do not affect the meaning of the code (white-space, formatting, missing semi-colons, etc)"),
            SupportedType::Test => CommitType::new("test", "Adding missing tests or correcting existing tests"),
            SupportedType::Wip => CommitType::new("wip", "An incomplete commit purely with the intent to push code off a local machine"),
        }
    }
}

#[derive(Debug)]
struct Ticket {
    team: String,
    number: i8,
    link: String,
}
impl Ticket {
    fn new(team: &str, number: i8, link: String) -> Ticket {
        Ticket {
            team: team.to_string(),
            number,
            link,
        }
    }
}
