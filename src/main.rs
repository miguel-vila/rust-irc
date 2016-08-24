extern crate bufstream;
#[macro_use] extern crate mdo;

pub mod parsing;
pub mod commands;
pub mod irc_error;

use std::net::{ TcpListener, TcpStream };
use std::thread;
use std::io::{ Write, BufRead };
use bufstream::BufStream;
//use std::error::Error;
use commands::{ Command, JoinCommandData };
use irc_error::IrcError;
use parsing::parse_command;
use std::collections::HashMap;
use std::sync::{ Arc , Mutex };

struct UserData {
    user: String,
    mode: i32,
    realname: String
}


struct IrcClient {
    stream: BufStream<TcpStream>,
    nick_name: Option<String>,
    user_data: Option<UserData>,
}

#[derive(Debug)]
struct IrcServer {
    channels_and_users : HashMap<String, Vec<String>>,
    n: i32,
}

fn leave_all(client: &mut IrcClient) {
    
}

fn handle_command(command: Command, client: &mut IrcClient, server: &Arc<Mutex<IrcServer>>) -> Result<String, IrcError> {
    println!("COMMAND : {:?}", command);
    match command {
        Command::Nick(nick_name) => {
            client.nick_name = Some( nick_name );
            Ok("RESPONSE".to_string())  
        },
        Command::Ping(msg) => {
            Ok("PONG ".to_string() + &msg)
        },
        Command::User(user, mode, realname) => {
            client.user_data = Some( UserData{ user: user, mode: mode, realname: realname} );
            Ok("dsafa".to_string())
        },
        Command::Join(join_command_data) => {
            match join_command_data {
                JoinCommandData::LeaveAll => {
                    leave_all(client);
                    Ok("Ok".to_string())
                },
                JoinCommandData::JoinChannel(channel_name) => {
                    let mut server = (*server).lock().unwrap();
                    println!("Before: {:?}", server.channels_and_users);
                    let mut new_users = Vec::new();
                    new_users.push(channel_name.clone());
                    server.channels_and_users.insert(channel_name, new_users);
                    println!("After: {:?}", server.channels_and_users);
                    Ok("Ok".to_string())
                }
            }
        }
        /*
        not_implemented => {
            Err( IrcError::NotImplemented )
        }
*/
    }
}

fn handle_client(client: &mut IrcClient, server: &Arc<Mutex<IrcServer>>) {
    println!("Received connection!");
    loop {
        let mut incoming_data = String::new();
        client.stream.read_line(&mut incoming_data);
        println!("Received: {:?}",incoming_data);
        let parse_result = parse_command(incoming_data);
        let response = match parse_result {
            Ok(command)    => handle_command(command, client, server),
            Err(irc_error) => Err( IrcError::Parse(format!("Error parsing command: {:?}",irc_error)) ),
        };
        match response {
            Ok(response) => {
                client.stream.write(response.as_bytes());
            }
            Err(irc_error) => {
                println!("Error processing command: {:?}", irc_error);
            }
        }
        client.stream.write("\n".as_bytes());
        client.stream.flush();
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6666").unwrap();
    let mut server = IrcServer { channels_and_users : HashMap::new(), n:0 } ;
    //    server.n =2;
    let shared_server = Arc::new(Mutex::new(server));
    println!("Starting server!");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let shared_server = shared_server.clone();
                thread::spawn(move || {
                    let bufstream = BufStream::new(stream);
                    let mut client = IrcClient { stream: bufstream ,
                                                 nick_name : None ,
                                                 user_data : None
                    };
                    handle_client(&mut client, &shared_server)
                });
            }
            Err(e) => {
                println!("Error {:?}",e);
            }
        }
    }
    drop(listener);
}
