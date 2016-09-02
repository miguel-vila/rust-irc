#[derive(Debug)]
pub enum IrcError {
    IncompleteCommand(String),
    Parse(String),
    NotImplemented,
    UserHasNoNickName
}

