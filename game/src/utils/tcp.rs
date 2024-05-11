use std::{
    io::{self, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

// Server
pub fn server<F>(handler: F)
where
    F: Fn(TcpStream),
{
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind address");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handler(stream);
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
        let buffer = read_until_separator(&mut stream).expect("Failed to read from stream");
        // let mut buffer = vec![0; 1];
        // stream.read(&mut buffer).unwrap();
        let response = String::from_utf8_lossy(&buffer).to_string();
        println!("==================Received==================");
        print!("{}", response);
    }
}

fn read_until_separator(stream: &mut TcpStream) -> io::Result<Vec<u8>> {
    const SEPARATOR: u8 = 0x7e; // ETX (End Of Text) separator byte
    let mut content_buffer = Vec::new();

    // Read data into the buffer until the separator is found
    loop {
        let mut chunk = [0; 1]; // Buffer for reading chunks of data
        let bytes_read = stream.read(&mut chunk).unwrap();
        if bytes_read == 0 {
            break; // No more data available
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

pub fn write_to_stream(stream: &mut TcpStream, data: String, end: bool) -> io::Result<()> {
    let mut data = data.as_bytes().to_vec();
    if end {
        data.push(0x7e);
    }
    stream.write(&data).unwrap();
    stream.flush().unwrap();
    Ok(())
}