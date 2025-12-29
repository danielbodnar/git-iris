use anyhow::Result;
use git_iris::cli;

/// Main entry point for the application
#[tokio::main]
async fn main() -> Result<()> {
    if let Err(e) = git_iris::logger::init() {
        eprintln!("Warning: Failed to initialize logging: {e}");
    }
    match cli::main().await {
        Ok(()) => Ok(()),
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
}
