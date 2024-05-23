#![allow(unused_imports)]
use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    time::{Duration, SystemTime},
};

#[derive(Debug)]
enum ReCommand {
    Set(String, String, u64),
    Get(String),
    Echo(String),
    Ping,
}

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || handle_connection(stream));
                println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
fn parser(data: &str) -> Vec<&str> {
    let s: Vec<&str> = data.split("$").collect();
    let mut v: Vec<&str> = [].to_vec();
    for i in 1..s.len() {
        let k: Vec<&str> = s[i].split("\r\n").collect();
        v.push(k[1]);
    }
    v
}

fn process_command(x: Vec<&str>) -> Option<ReCommand> {
    if x.is_empty() {
        return None;
    }
    let command = match x[0] {
        "echo" => Some(ReCommand::Echo(x[1].to_string())),
        "get" => Some(ReCommand::Get(x[1].to_string())),
        "set" => Some(ReCommand::Set(x[1].to_string(), x[2].to_string(), 10)),
        "ping" => Some(ReCommand::Ping),
        _ => None,
    };
    command
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    let mut data_holder: HashMap<String, (String, SystemTime)> = HashMap::new();
    loop {
        let data = stream.read(&mut buf).unwrap();
        if data == 0 {
            break;
        }
        let y = String::from_utf8_lossy(&buf[..data]);
        let x = parser(&y);

        if let Some(command) = process_command(x.clone()) {
            match command {
                ReCommand::Ping => {
                    stream.write(b"+PONG\r\n").unwrap();
                }

                ReCommand::Echo(mess) => {
                    let _ = stream
                        .write(format!("${}\r\n{}\r\n", mess.len(), mess).as_bytes())
                        .unwrap();
                }
                ReCommand::Get(key) => {
                    let _ = match data_holder.get(&key) {
                        Some(value) => stream
                            .write(format!("${}\r\n{}\r\n", value.0.len(), value.0).as_bytes()),
                        None => stream.write(b"$-1\r\n"),
                    };
                }
                ReCommand::Set(key, value, num) => {
                    let duration = Duration::from_millis(num);
                    let exp_time = SystemTime::now() + duration;
                    data_holder.insert(key.to_string(), (value.to_string(), exp_time));
                    let _ = stream.write(b"+OK\r\n").unwrap();
                }
            }
        } else {
            let _ = stream.write(b"$-1\r\n");
        }
    }
}
