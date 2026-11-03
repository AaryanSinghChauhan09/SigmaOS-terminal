//! SigmaOS-terminal - SigmaOS Component CLI
//!
//! Command-line interface for SigmaOS-terminal component.

use anyhow::Result;
use clap::{Parser, Subcommand};
use log::info;
use terminal::{Component, ComponentStatus, Config, terminal};

#[derive(Parser)]
#[command(name = "terminal")]
#[command(about = "SigmaOS SigmaOS-terminal component", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the component
    Init {
        /// Enable debug mode
        #[arg(short, long)]
        debug: bool,
    },
    /// Start the component
    Start,
    /// Stop the component
    Stop,
    /// Get component status
    Status,
    /// Run the component
    Run {
        /// Enable debug mode
        #[arg(short, long)]
        debug: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    let config = Config::default();
    let mut component = terminal::new(config);

    match cli.command {
        Commands::Init { debug } => {
            if debug {
                println!("Initializing SigmaOS-terminal in debug mode...");
            } else {
                println!("Initializing SigmaOS-terminal...");
            }
            component.initialize()?;
            println!("SigmaOS-terminal initialized successfully");
        }
        Commands::Start => {
            component.initialize()?;
            component.start()?;
            println!("SigmaOS-terminal started successfully");
        }
        Commands::Stop => {
            component.stop()?;
            println!("SigmaOS-terminal stopped successfully");
        }
        Commands::Status => {
            let status = component.status();
            println!("SigmaOS-terminal status: {:?}", status);
        }
        Commands::Run { debug } => {
            if debug {
                println!("Running SigmaOS-terminal in debug mode...");
            } else {
                println!("Running SigmaOS-terminal...");
            }
            component.initialize()?;
            component.start()?;
            println!("SigmaOS-terminal is running. Press Ctrl+C to stop.");
            
            // Keep running
            tokio::runtime::Runtime::new()?.block_on(async {
                tokio::signal::ctrl_c().await.unwrap();
                info!("Received Ctrl+C, shutting down...");
            });
            
            component.stop()?;
            println!("SigmaOS-terminal stopped");
        }
    }

    Ok(())
}
