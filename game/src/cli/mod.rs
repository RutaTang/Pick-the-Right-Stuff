use clap::{arg, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(arg_required_else_help(true))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Start the game server")]
    Serve {
        /// The type of the game: "Zero", "Finite" or "Infinite"
        #[arg(short, long)]
        mode: String,
    },
     #[command(about = "Start the game client")]
    Client {
         #[arg(short, long)]
         port: usize,
    },
}
