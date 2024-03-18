use std::env;

use tokio::{
    fs,
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

pub async fn handle_requests(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let mut read_lines = buf_reader.lines();
    let mut http_request = Vec::new();
    let directory = env::args().nth(2);

    while let Some(line) = read_lines.next_line().await.unwrap() {
        if line.is_empty() {
            break;
        }
        http_request.push(line);
    }

    if http_request.is_empty() {
        stream
            .write_all("HTTP/1.1 400 Bad Request".as_bytes())
            .await
            .unwrap();
        println!("Invalid Request Recvd");
        return;
    }

    let start_line = &http_request[0];
    let path = start_line.split(' ').nth(1).unwrap();

    let response = match path {
        "/" => String::from("HTTP/1.1 200 OK\r\n\r\n"),

        path if path.starts_with("/echo/") => {
            let echo_path = path.trim_start_matches("/echo/").to_owned();
            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                echo_path.as_bytes().len(),
                echo_path
            )
        }

        path if path.starts_with("/user-agent") => {
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
        }

        path if path.starts_with("/files") => match directory {
            Some(dir) => {
                let file_name = path.trim_start_matches("/files/").to_owned();
                let contents = fs::read_to_string(format!("{}/{}", dir, file_name)).await;
                match contents {
                    Ok(x) => format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}",
                    x.as_bytes().len(),
                    x
                ),
                    Err(_) => String::from("HTTP/1.1 404 Not Found\r\n\r\n"),
                }
            }
            None => {
                println!("No directory specified");
                String::from("HTTP/1.1 404 Not Found\r\n\r\n")
            }
        },
        _ => String::from("HTTP/1.1 404 Not Found\r\n\r\n"),
    };

    println!("Responding with {}", response);
    stream.write_all(response.as_bytes()).await.unwrap();
}
