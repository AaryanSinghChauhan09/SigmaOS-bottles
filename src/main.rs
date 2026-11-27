use bottles::{BottleConfig, BottleEnvironment, BottleManager};
use clap::{Parser, Subcommand};
use std::process;

#[derive(Parser)]
#[command(name = "sigmaos-bottles", about = "SigmaOS Wine-Bottle Manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Print a sample gaming bottle configuration JSON
    Sample,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Sample => {
            let mut mgr = BottleManager::new();
            let b = BottleConfig::new("Gaming", BottleEnvironment::Gaming, "/var/lib/bottles/gaming")
                .unwrap_or_else(|e| {
                    eprintln!("Error: {}", e);
                    process::exit(1);
                });
            let _ = mgr.create_bottle(b);
            println!("{}", mgr.to_json().unwrap());
        }
    }
}
