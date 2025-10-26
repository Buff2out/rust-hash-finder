use clap::Parser;
use rust_hash_finder::find_hashes;
use tracing::{info};
use tracing_subscriber::EnvFilter;
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'N', long)]
    zeros: usize,
    
    #[arg(short = 'F', long)]
    results: usize,
    
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> ExitCode {
    let args = Args::parse();
    
    if args.zeros == 0 || args.results == 0 {
        eprintln!("Error: Both N and F must be greater than 0");
        return ExitCode::FAILURE;
    }
    
    let default_level = if args.verbose { "debug" } else { "info" };
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(format!("rust_hash_finder={}", default_level)));
    
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .init();
    
    info!("Hash Finder starting...");
    info!("Configuration: N={}, F={}", args.zeros, args.results);
    
    let results = find_hashes(args.zeros, args.results);
    
    for (num, hash) in results {
        println!("{}, \"{}\"", num, hash);
    }
    
    info!("Hash Finder completed successfully");
    ExitCode::SUCCESS
}
