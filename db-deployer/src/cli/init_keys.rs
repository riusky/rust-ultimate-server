use anyhow::Result;
use clap::Args;
use console::style;

use crate::crypto::{generate_key_pair, save_private_key, save_public_key};
use std::path::Path;

#[derive(Args)]
pub struct InitKeysArgs {
    /// Force overwrite existing keys
    #[arg(short, long)]
    pub force: bool,
}

pub fn execute(args: InitKeysArgs, config_path: &str) -> Result<()> {
    println!("{}", style("=".repeat(60)).cyan());
    println!("{}", style("  RSA Key Generation").cyan().bold());
    println!("{}", style("=".repeat(60)).cyan());
    println!();

    // Load config
    let config = load_config_for_keys(config_path)?;

    let private_path = Path::new(&config.keys.private_key);
    let public_path = Path::new(&config.keys.public_key);

    // Check if keys already exist
    if !args.force {
        if private_path.exists() {
            anyhow::bail!(
                "Private key already exists: {}\nUse --force to overwrite",
                config.keys.private_key
            );
        }
        if public_path.exists() {
            anyhow::bail!(
                "Public key already exists: {}\nUse --force to overwrite",
                config.keys.public_key
            );
        }
    }

    println!("Generating 2048-bit RSA key pair...");

    let (private_key, public_key) = generate_key_pair()?;

    // Save keys
    save_private_key(&private_key, private_path)?;
    println!("  Private key saved: {}", style(&config.keys.private_key).green());

    save_public_key(&public_key, public_path)?;
    println!("  Public key saved: {}", style(&config.keys.public_key).green());

    println!();
    println!("{}", style("Keys generated successfully!").green().bold());
    println!();
    println!("Important:");
    println!(
        "  - {} the private key (add to .gitignore)",
        style("DO NOT commit").red().bold()
    );
    println!("  - The public key can be committed to git");
    println!();

    Ok(())
}

fn load_config_for_keys(config_path: &str) -> Result<crate::config::Config> {
    let path = std::path::Path::new(config_path);
    if !path.exists() {
        anyhow::bail!(
            "Configuration file not found: {}\n\
             Create one from the example or run with --config <path>",
            config_path
        );
    }

    let content = std::fs::read_to_string(path)?;
    let config: crate::config::Config = toml::from_str(&content)?;
    Ok(config)
}
