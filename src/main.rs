// todo remove
#![allow(unused)]

#[macro_use]
extern crate lazy_static;

use clap::{ Parser, Subcommand };
use dotenvy::dotenv;
use std::env::var;
use std::path::PathBuf;
use tokio::fs;

mod cli;
mod db;
mod github;

use cli::Cli;
use db::DatabaseThing;

pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result {
	let _ = dotenv();

	let db_filename = "state.ron";
	let token = var("GITHUB_TOKEN")
		.map_err(|e| format!("error in fetching GITHUB_TOKEN: {}", e))?;

	let cli = Cli::parse();
	let db = DatabaseThing::new("state.ron").await?;

	use cli::Subcommands::*;
	match cli.command {
		ReadPackageData { files } => {
			cli::read_package_data(db, files).await?;
		}
	}

	Ok(())
}
