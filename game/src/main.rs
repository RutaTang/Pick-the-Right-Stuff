use std::io::{Read, Write};
use std::net::TcpListener;

use game::logic::engine;

// Server
fn server() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind address");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 1024];
                stream
                    .read(&mut buffer)
                    .expect("Failed to read from stream");
                let message = String::from_utf8_lossy(&buffer).to_string();
                let modified_message = format!("Server says: {}", message);
                stream
                    .write_all(modified_message.as_bytes())
                    .expect("Failed to write to stream");

                if message.trim() == "bye" {
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

fn main() {
    // Start the server in a separate thread
    let server_thread = std::thread::spawn(engine::start);

    // Wait for the server thread to finish
    server_thread.join().expect("Failed to join server thread");
}
