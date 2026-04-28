use anyhow::Result;
use git_iris::{cli, crypto};

/// Main entry point for the application
#[tokio::main]
async fn main() -> Result<()> {
    crypto::install_default_crypto_provider();

    // Logger init is deferred to cli::main() so --log flag can raise the tracing level
    match cli::main().await {
        Ok(()) => Ok(()),
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
}
