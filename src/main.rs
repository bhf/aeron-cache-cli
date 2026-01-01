mod commands;
mod cacherequests;
mod cacheresponses;

use clap::{Parser, Subcommand};
use dialoguer::Confirm;
use reqwest::blocking::Client;
use std::error::Error;
use std::env;

use commands::*;

#[derive(Parser)]
#[command(
    name = "cache-cli",
    about = "A CLI for interacting with Aeron Cache - https://github.com/bhf/aeron-cache"
)]
struct Cli {
    #[arg(
        long,
        default_value = "http://localhost:7070/api/v1/cache",
        help = "Aeron Cache API base URL"
    )]
    api_url: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Create a new cache")]
    Create {
        #[arg(help = "Name of the cache to create")]
        name: String,
    },

    #[command(about = "Insert an item into a cache")]
    Insert {
        #[arg(help = "Name of the cache to insert into")]
        name: String,
        #[arg(help = "Key to insert the item")]
        key: String,
        #[arg(help = "Value to be inserted")]
        value: String,
    },

    #[command(about = "Get an item from a cache")]
    Get {
        #[arg(help = "Name of the cache to get the item from")]
        name: String,
        #[arg(help = "Key of the item we want to get")]
        key: String,
    },

    #[command(about = "Remove an item from a cache")]
    Remove {
        #[arg(help = "Name of the cache to remove from")]
        name: String,
        #[arg(help = "Key of the item we want to remove")]
        key: String,
    },

    #[command(about = "Delete a cache")]
    Delete {
        #[arg(help = "Name of the cache to delete")]
        name: String,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let rest_client = Client::new();

    // Check CLI arg, then env var, then default
    let aeron_cache_api_url = if cli.api_url != "http://localhost:7070/api/v1/cache" {
        cli.api_url
    } else if let Ok(val) = env::var("AERON_CACHE_API_URL") {
        val
    } else {
        "http://localhost:7070/api/v1/cache".to_string()
    };

    match cli.command {
        Commands::Create { name: cache_name } => {
            process_create_cache(&rest_client, &aeron_cache_api_url, &cache_name)?;
        }
        Commands::Insert { name: cache_name, key, value } => {
            process_insert_item(&rest_client, &aeron_cache_api_url, &cache_name, &key, value)?;
        }
        Commands::Get { name: cache_name, key } => {
            process_get_item(&rest_client, &aeron_cache_api_url, &cache_name, &key)?;
        }
        Commands::Remove { name: cache_name, key } => {
            process_remove_item(rest_client, &aeron_cache_api_url, &cache_name, &key)?;
        }
        Commands::Delete { name } => {
            if Confirm::new()
                .with_prompt(format!(
                    "Are you sure you want to delete cache '{}'? This action cannot be undone.",
                    name
                ))
                .interact()?
            {
                process_delete_cache(&rest_client, &aeron_cache_api_url, &name)?;
            } else {
                println!("Cache '{}' not deleted", name)
            }
        }
    }
    Ok(())
}

