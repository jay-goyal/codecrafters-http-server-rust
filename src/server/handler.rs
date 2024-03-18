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
    let path = start_line.split(' ').nth(1).unwrap();

    let response = if path == "/" {
        String::from("HTTP/1.1 200 OK\r\n\r\n")
    } else if path.starts_with("/echo/") {
        let echo_path = path.trim_start_matches("/echo/").to_owned();
        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            echo_path.as_bytes().len(),
            echo_path
        )
    } else if path.starts_with("/user-agent") {
        let mut user_agent = None;
        for line in http_request {
            if line.starts_with("User-Agent:") {
                user_agent = Some(line.split(' ').nth(1).unwrap().to_owned());
                break;
            }
        }
        match user_agent {
            Some(x) => format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                x.as_bytes().len(),
                x
            ),
            None => String::from("HTTP/1.1 404 Not Found\r\n\r\n"),
        }
    } else {
        String::from("HTTP/1.1 404 Not Found\r\n\r\n")
    };

    stream.write_all(response.as_bytes()).unwrap();
}
