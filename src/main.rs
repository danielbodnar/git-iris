use anyhow::Result;
use git_iris::cli;

/// Main entry point for the application
#[tokio::main]
async fn main() -> Result<()> {
    // Logger init is deferred to cli::main() so --log flag can raise the tracing level
    match cli::main().await {
        Ok(()) => Ok(()),
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
}
