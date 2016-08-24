#[derive(Debug)]
pub enum JoinCommandData {
    LeaveAll,
    JoinChannel(String)
}

#[derive(Debug)]
pub enum Command {
    Nick(String),
    User(String, i32 /*mode -> maybe create own type?*/, String),
    Ping(String),
    Join(JoinCommandData)
}
