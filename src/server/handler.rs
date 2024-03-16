use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

pub fn handle_requests(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let start_line = &http_request[0];
    let method = start_line.split(' ').nth(1).unwrap();

    let response = match method {
     "/" => "HTTP/1.1 200 OK\r\n\r\n",
        _ => "HTTP/1.1 404 Not Found\r\n\r\n"
    };
    stream.write_all(response.as_bytes()).unwrap();
}
