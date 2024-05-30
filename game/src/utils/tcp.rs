use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    sync::Arc,
};
use std::fmt::Display;

const SEPARATOR: u8 = 0x0a;

// Server
pub fn server<F>(port: usize, handler: F)
    where
        F: Fn(TcpStream) + Send + Sync + 'static,
{
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).expect("Failed to bind address");
    println!("Server listening on port {}", port);
    let handler = Arc::new(handler);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New player connected!");
                let handler = Arc::clone(&handler);
                std::thread::spawn(move || {
                    handler(stream);
                    println!("Player disconnected!");
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

// Client
pub fn client() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect to server");

    loop {
        let buffer = read_until_separator(&mut stream).unwrap_or_else(|_| {
            println!("Connection closed");
            std::process::exit(0);
        });
        let response = String::from_utf8_lossy(&buffer).to_string();
        let response = response.trim();
        let response = Data::from_json(&response);
        if response.require_input() {
            println!("{}", response.content());
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let data = Data::new(false, input.trim().to_string());
            write_to_stream(&mut stream, data).unwrap();
        } else if response.content().contains("Game Over!") {
            println!("{}", response.content());
            break;
        } else {
            println!("{}", response.content());
        }
    }
}

/// read_until_separator reads data from the stream until a separator is found
pub fn read_until_separator(stream: &mut TcpStream) -> io::Result<Vec<u8>> {
    let mut content_buffer = Vec::new();

    // Read data into the buffer until the separator is found
    loop {
        let mut chunk = [0; 1]; // Buffer for reading chunks of data
        let bytes_read = stream.read(&mut chunk).unwrap();
        if bytes_read == 0 {
            // return an error if the stream is closed
            return Err(io::Error::new(io::ErrorKind::Other, "Stream closed"));
        }

        // Check for the separator in the chunk and handle partial reads
        if chunk[0] == SEPARATOR {
            break; // Separator found
        } else {
            content_buffer.push(chunk[0]);
        }
    }

    Ok(content_buffer)
}

/// write_to_stream writes data to the stream and appends a separator at the end
pub fn write_to_stream(stream: &mut TcpStream, data: Data) -> io::Result<()> {
    let mut data = data.to_json().as_bytes().to_vec();
    data.push(SEPARATOR);
    stream.write(&data).unwrap();
    stream.flush().unwrap();
    Ok(())
}


/// Data to be sent over the network
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Data {
    require_input: bool,
    content: String,
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl Data {
    pub fn new(require_input: bool, content: String) -> Self {
        Self {
            require_input,
            content,
        }
    }
    pub fn from_json(json: &str) -> Self {
        serde_json::from_str(json).unwrap()
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    pub fn require_input(&self) -> bool {
        self.require_input
    }
    pub fn content(&self) -> &str {
        &self.content
    }
}