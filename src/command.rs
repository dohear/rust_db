pub enum MetaCommand {
    Exit,
    Unrecognized(String),
}

pub enum Statement {
    Insert,
    Select,
    Unrecognized(String),
}

pub enum Command {
    MetaCommand(MetaCommand),
    Statement(Statement),
}

impl Command {
    pub fn parse(input: &[String]) -> Self {
        match input[0].chars().next() {
            Some('.') => Command::MetaCommand(MetaCommand::parse(input)),
            _ => Command::Statement(Statement::parse(input)),
        }
    }
}

impl MetaCommand {
    pub fn parse(input: &[String]) -> Self {
        match input[0].as_str() {
            ".exit" => MetaCommand::Exit,
            _ => MetaCommand::Unrecognized(input[0].clone()),
        }
    }
}

impl Statement {
    pub fn parse(input: &[String]) -> Self {
        match input[0].as_str() {
            "insert" => Statement::Insert,
            "select" => Statement::Select,
            _ => Statement::Unrecognized(input[0].clone()),
        }
    }
}
