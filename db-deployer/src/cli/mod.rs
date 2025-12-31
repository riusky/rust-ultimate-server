mod deploy;
mod init_keys;
mod status;

pub use deploy::DeployArgs;
pub use init_keys::InitKeysArgs;
pub use status::StatusArgs;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "db-deployer")]
#[command(about = "Database deployment and upgrade tool with RSA signature verification", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Path to configuration file
    #[arg(short, long, default_value = "db-deployer.toml")]
    pub config: String,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Deploy or upgrade database
    Deploy(DeployArgs),

    /// Show current deployment status
    Status(StatusArgs),

    /// Generate RSA key pair
    InitKeys(InitKeysArgs),
}

impl Cli {
    pub async fn execute(self) -> anyhow::Result<()> {
        match self.command {
            Commands::Deploy(args) => deploy::execute(args, &self.config).await,
            Commands::Status(args) => status::execute(args, &self.config).await,
            Commands::InitKeys(args) => init_keys::execute(args, &self.config),
        }
    }
}
