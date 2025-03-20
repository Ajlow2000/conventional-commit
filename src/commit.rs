use strum::EnumIter;

#[derive(Debug)]
pub struct CommitMsg {
    pub commit_type: String,//SupportedType,
    pub scope: Option<String>,
    pub desc: String,
    // pub body: Option<String>,
    pub related_ticket: Option<String>,//Option<Ticket>,
    pub breaking_reason: Option<String>,
}

impl CommitMsg {
    pub fn commit(&self) -> String {
        let formated_msg = format!("{}{}: {}\n\n\nTICKET: {}\nBREAKING CHANGE: {}",
            self.commit_type,
            self.scope.clone().unwrap_or("".to_string()),
            self.desc,
            self.related_ticket.clone().unwrap_or("".to_string()),
            self.breaking_reason.clone().unwrap_or("".to_string()),
        );
        formated_msg
    }
}

#[derive(Debug)]
pub struct CommitType {
    pub value: String,
    pub description: String,
}
impl CommitType {
    fn new(value: &str, description: &str) -> CommitType {
        CommitType {
            value: value.to_string(), 
            description: description.to_string(),
        }
    }
}

#[derive(Debug, EnumIter)]
pub enum SupportedType {
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
    pub fn to_commit_type(&self) -> CommitType {
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
