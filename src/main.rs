use tokio::net::TcpListener;

use crate::server::handler::handle_requests;
mod server;

#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").await.unwrap();

    loop {
        match listener.accept().await {
            Ok(x) => {
                tokio::spawn(handle_requests(x.0));
            }
            Err(e) => {
                println!("Error {}", e);
            }
        }
    }
}
