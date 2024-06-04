use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7567").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream)
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines() // getting all the lines in the request
        .map(|result| result.unwrap()) // TODO: Handling them gracefully maybe?
        .take_while(|line| !line.is_empty()) // Reading till an empty line cuz spec
        .collect();

    println!("Request: {:#?}", http_request);
}
