use clap::Parser;
use game::{cli, logic::engine::start, utils::tcp::{client, server}};


// ...

fn main() {
    let cli = cli::Cli::parse();
    match cli.command {
        Some(cli::Commands::Start { mode }) => {
            if mode == "server" {
                let server_thread = std::thread::spawn(||{server(start)});
                println!("Game server is running!");
                server_thread.join().expect("Failed to join server thread");
            } else if mode == "client" {
                let client_thread = std::thread::spawn(client);
                println!("Game client is running!");
                client_thread.join().expect("Failed to join client thread");
            } else {
                println!("Invalid mode");
            }
        }
        None => {}
    }
}
