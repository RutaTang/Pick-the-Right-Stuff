use clap::Parser;
use game::{cli,utils::tcp::{client, server}};
use game::logic::engine::start;
use game::logic::engine::GameMode::{Finite, Zero};

fn main() {
    let cli = cli::Cli::parse();
    match cli.command {
        Some(cli::Commands::Serve { mode }) => {
            if mode == "zero" {
                let server_thread = std::thread::spawn(|| { server(start(Zero)) });
                println!("Game server is running in Zero Belief History mode!");
                server_thread.join().expect("Failed to join server thread");
            } else if mode == "finite" { 
                let server_thread = std::thread::spawn(|| { server(start(Finite)) });
                println!("Game server is running in Finite Belief History mode!");
                server_thread.join().expect("Failed to join server thread");
            }
            else {
                println!("Invalid mode, choose either 'zero' or 'finite'!");
            }
        }
        Some(cli::Commands::Client { }) => {
            let client_thread = std::thread::spawn(client);
            println!("Game client is running!");
            client_thread.join().expect("Failed to join client thread");
        }
        None => {}
    }
}
