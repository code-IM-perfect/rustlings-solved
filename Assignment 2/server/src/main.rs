use std::{
    fs,
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

    // println!("Request: {:#?}", http_request);

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("index.html").unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap(); // TODO: Handling gracefully

    let req_line_split: Vec<_> = http_request[0].trim().split_whitespace().collect();
    println!("Responded to request to {}", req_line_split[1]);
}
