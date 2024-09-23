use std::net::TcpListener;
use zero2prod::startup;
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    startup::run(TcpListener::bind("127.0.0.1:8000")?)?.await
}