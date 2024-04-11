#![allow(unused_imports)]
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

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

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 1024];

    loop {
        let data = stream.read(&mut buf).unwrap();
        if data == 0 {
            break;
        }
        let y = String::from_utf8_lossy(&buf[..data]);
        let x = parser(&y);
        if x.len() == 0 {
            break;
        } else if x[0] == "echo" {
            let _ = stream
                .write(format!("${}\r\n{}\r\n", x[1].len(), x[1]).as_bytes())
                .unwrap();
        } else {
            stream.write(b"+PONG\r\n").unwrap();
        }
        println!("{:?}", x);
    }
}
