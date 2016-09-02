use mdo::result::{ ret, bind };
use std::str::SplitWhitespace;
use commands::{ Command, JoinCommandData };
use irc_error::IrcError;

fn parse_nick(mut params: SplitWhitespace) -> Result<Command, IrcError> {
    params.next()
        .map( |nick| { Command::Nick( nick.to_string() ) })
        .ok_or( IrcError::IncompleteCommand( "Incomplete NICK command".to_string() ) )
}

fn parse_ping(mut params: SplitWhitespace) -> Result<Command, IrcError> {
    let content = params.next().unwrap_or_else(|| "");
    Ok( Command::Ping(content.to_string()) )
}

fn parse_number(str: &str, error_message: &str) -> Result<i32, IrcError> {
    match str.parse::<i32>() {
        Ok(num) => Ok(num),
        Err(_) => Err( IrcError::IncompleteCommand( error_message.to_string() ) )
    }
}


fn parse_user(mut params: SplitWhitespace) -> Result<Command, IrcError> {
    let user_result = params.next().ok_or( IrcError::IncompleteCommand( "Missing 'user' for USER command".to_string() ) );
    let mode_str_result = params.next().ok_or( IrcError::IncompleteCommand( "Missing 'mode' for USER command".to_string() ) );
    params.next();
    let realname_opt = params.next().ok_or( IrcError::IncompleteCommand( "Missing 'realname' for USER command".to_string() ) );
    
    mdo! {
        user     =<< user_result;
        mode_str =<< mode_str_result;
        mode     =<< parse_number(mode_str, "'mode' parameter must be an integer");
        realname =<< realname_opt;
        ret ret(Command::User(user.to_string(), mode, realname.to_string()))
    }
}

fn parse_join(mut params: SplitWhitespace) -> Result<Command, IrcError> {
    let channel_name_opt = params.next();
    match channel_name_opt {
        Some("0") => Ok( Command::Join( JoinCommandData::LeaveAll ) ),
        Some(channel_name) => Ok( Command::Join( JoinCommandData::JoinChannel(channel_name.to_string())) ),
        None => Err( IrcError::IncompleteCommand("Missing data after JOIN command".to_string()) )
    }
}
    

pub fn parse_command(command_line: String) -> Result<Command, IrcError> {
    let mut parts = command_line.split_whitespace();
    match parts.next() {
        Some( "NICK" ) => parse_nick(parts),
        Some( "PING" ) => parse_ping(parts),
        Some( "USER" ) => parse_user(parts),
        Some( "JOIN" ) => parse_join(parts),
        Some( other  ) => Err(IrcError::Parse(format!("Unrecognized command: {}", other))),
        None => Err(IrcError::IncompleteCommand("".to_string()))
    }
}

