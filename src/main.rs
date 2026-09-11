mod commands;
mod cacherequests;
mod cacheresponses;
mod counterscommands;
mod countersrequests;
mod countersresponses;

use clap::{Parser, Subcommand};
use dialoguer::Confirm;
use reqwest::blocking::Client;
use std::error::Error;
use std::env;

use commands::*;
use counterscommands::*;

#[derive(Parser)]
#[command(
    name = "cache-cli",
    about = "A CLI for interacting with Aeron Cache - https://github.com/bhf/aeron-cache"
)]
struct Cli {
    #[arg(
        long,
        default_value = "http://localhost:7070/api/v1",
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

    #[command(about = "Insert an item with a time-to-live into a cache")]
    InsertTimed {
        #[arg(help = "Name of the cache to insert into")]
        name: String,
        #[arg(help = "Key to insert the item")]
        key: String,
        #[arg(help = "Value to be inserted")]
        value: String,
        #[arg(help = "Time-to-live in milliseconds")]
        ttl: i64,
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

    #[command(about = "Cancel a scheduled removal of an item")]
    CancelRemoval {
        #[arg(help = "Name of the cache")]
        name: String,
        #[arg(help = "Key of the item")]
        key: String,
    },

    #[command(about = "Delete a cache")]
    Delete {
        #[arg(help = "Name of the cache to delete")]
        name: String,
        #[arg(short, long, help = "Automatically confirm deletion")]
        yes: bool,
    },

    #[command(about = "Get all items from a cache")]
    GetCache {
        #[arg(help = "Name of the cache")]
        name: String,
    },

    #[command(about = "Clear all items from a cache")]
    ClearCache {
        #[arg(help = "Name of the cache to clear")]
        name: String,
    },

    #[command(about = "List all caches")]
    ListCaches,

    #[command(about = "Get global cache statistics")]
    Stats,

    #[command(about = "Create a new counter cache")]
    CreateCounterCache {
        #[arg(help = "Name of the counter cache to create")]
        name: String,
    },

    #[command(about = "Put a counter into a counter cache")]
    PutCounter {
        #[arg(help = "Name of the counter cache to put into")]
        name: String,
        #[arg(help = "Key of the counter")]
        key: String,
        #[arg(help = "Value of the counter")]
        value: i64,
    },

    #[command(about = "Put a counter with a time-to-live into a counter cache")]
    PutTimedCounter {
        #[arg(help = "Name of the counter cache to put into")]
        name: String,
        #[arg(help = "Key of the counter")]
        key: String,
        #[arg(help = "Value of the counter")]
        value: i64,
        #[arg(help = "Time-to-live in milliseconds")]
        ttl: i64,
    },

    #[command(about = "Get a single counter from a counter cache")]
    GetCounter {
        #[arg(help = "Name of the counter cache to get the counter from")]
        name: String,
        #[arg(help = "Key of the counter we want to get")]
        key: String,
    },

    #[command(about = "Delete a single counter from a counter cache")]
    DeleteCounter {
        #[arg(help = "Name of the counter cache to delete from")]
        name: String,
        #[arg(help = "Key of the counter we want to delete")]
        key: String,
    },

    #[command(about = "Increment a counter")]
    IncrementCounter {
        #[arg(help = "Name of the counter cache")]
        name: String,
        #[arg(help = "Key of the counter")]
        key: String,
        #[arg(default_value_t = 1, help = "Amount to increment by")]
        amount: i64,
    },

    #[command(about = "Decrement a counter")]
    DecrementCounter {
        #[arg(help = "Name of the counter cache")]
        name: String,
        #[arg(help = "Key of the counter")]
        key: String,
        #[arg(default_value_t = 1, help = "Amount to decrement by")]
        amount: i64,
    },

    #[command(about = "Set a counter to a given value")]
    SetCounter {
        #[arg(help = "Name of the counter cache")]
        name: String,
        #[arg(help = "Key of the counter")]
        key: String,
        #[arg(help = "Value to set the counter to")]
        value: i64,
    },

    #[command(about = "Cancel a scheduled removal of a counter")]
    CancelCounterRemoval {
        #[arg(help = "Name of the counter cache")]
        name: String,
        #[arg(help = "Key of the counter")]
        key: String,
    },

    #[command(about = "Get all counters from a counter cache")]
    GetCounterCache {
        #[arg(help = "Name of the counter cache")]
        name: String,
    },

    #[command(about = "Clear all counters from a counter cache")]
    ClearCounterCache {
        #[arg(help = "Name of the counter cache to clear")]
        name: String,
    },

    #[command(about = "Delete a counter cache")]
    DeleteCounterCache {
        #[arg(help = "Name of the counter cache to delete")]
        name: String,
        #[arg(short, long, help = "Automatically confirm deletion")]
        yes: bool,
    },

    #[command(about = "List all counter caches")]
    ListCounterCaches,

    #[command(about = "Get global counter cache statistics")]
    CounterStats,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let rest_client = Client::new();

    // Check CLI arg, then env var, then default
    let aeron_cache_api_url = if cli.api_url != "http://localhost:7070/api/v1" {
        cli.api_url
    } else if let Ok(val) = env::var("AERON_CACHE_API_URL") {
        val
    } else {
        "http://localhost:7070/api/v1".to_string()
    };

    match cli.command {
        Commands::Create { name: cache_name } => {
            process_create_cache(&rest_client, &aeron_cache_api_url, &cache_name)?;
        }
        Commands::Insert { name: cache_name, key, value } => {
            process_insert_item(&rest_client, &aeron_cache_api_url, &cache_name, &key, value)?;
        }
        Commands::InsertTimed { name: cache_name, key, value, ttl } => {
            process_put_timed_item(&rest_client, &aeron_cache_api_url, &cache_name, &key, &value, ttl)?;
        }
        Commands::Get { name: cache_name, key } => {
            process_get_item(&rest_client, &aeron_cache_api_url, &cache_name, &key)?;
        }
        Commands::Remove { name: cache_name, key } => {
            process_remove_item(rest_client, &aeron_cache_api_url, &cache_name, &key)?;
        }
        Commands::CancelRemoval { name: cache_name, key } => {
            process_cancel_item_removal(&rest_client, &aeron_cache_api_url, &cache_name, &key)?;
        }
        Commands::Delete { name, yes } => {
            if yes || Confirm::new()
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
        Commands::GetCache { name: cache_name } => {
            process_get_cache(&rest_client, &aeron_cache_api_url, &cache_name)?;
        }
        Commands::ClearCache { name: cache_name } => {
            process_clear_cache(&rest_client, &aeron_cache_api_url, &cache_name)?;
        }
        Commands::ListCaches => {
            process_list_caches(&rest_client, &aeron_cache_api_url)?;
        }
        Commands::Stats => {
            process_get_stats(&rest_client, &aeron_cache_api_url)?;
        }
        Commands::CreateCounterCache { name } => {
            process_create_counter_cache(&rest_client, &aeron_cache_api_url, &name)?;
        }
        Commands::PutCounter { name, key, value } => {
            process_put_counter(&rest_client, &aeron_cache_api_url, &name, &key, value)?;
        }
        Commands::PutTimedCounter { name, key, value, ttl } => {
            process_put_timed_counter(&rest_client, &aeron_cache_api_url, &name, &key, value, ttl)?;
        }
        Commands::GetCounter { name, key } => {
            process_get_counter(&rest_client, &aeron_cache_api_url, &name, &key)?;
        }
        Commands::DeleteCounter { name, key } => {
            process_delete_counter(&rest_client, &aeron_cache_api_url, &name, &key)?;
        }
        Commands::IncrementCounter { name, key, amount } => {
            process_increment_counter(&rest_client, &aeron_cache_api_url, &name, &key, amount)?;
        }
        Commands::DecrementCounter { name, key, amount } => {
            process_decrement_counter(&rest_client, &aeron_cache_api_url, &name, &key, amount)?;
        }
        Commands::SetCounter { name, key, value } => {
            process_set_counter(&rest_client, &aeron_cache_api_url, &name, &key, value)?;
        }
        Commands::CancelCounterRemoval { name, key } => {
            process_cancel_counter_removal(&rest_client, &aeron_cache_api_url, &name, &key)?;
        }
        Commands::GetCounterCache { name } => {
            process_get_counter_cache(&rest_client, &aeron_cache_api_url, &name)?;
        }
        Commands::ClearCounterCache { name } => {
            process_clear_counter_cache(&rest_client, &aeron_cache_api_url, &name)?;
        }
        Commands::DeleteCounterCache { name, yes } => {
            if yes || Confirm::new()
                .with_prompt(format!(
                    "Are you sure you want to delete counter cache '{}'? This action cannot be undone.",
                    name
                ))
                .interact()?
            {
                process_delete_counter_cache(&rest_client, &aeron_cache_api_url, &name)?;
            } else {
                println!("Counter cache '{}' not deleted", name)
            }
        }
        Commands::ListCounterCaches => {
            process_list_counter_caches(&rest_client, &aeron_cache_api_url)?;
        }
        Commands::CounterStats => {
            process_get_counter_stats(&rest_client, &aeron_cache_api_url)?;
        }
    }
    Ok(())
}
