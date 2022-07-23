//! stuff used by the CLI

use crate::db::{ self, DatabaseThing };
use crate::Result;
use clap::{ Parser, Subcommand };
use tokio::fs;

#[derive(Parser)]
#[clap(version)]
pub struct Cli {
	#[clap(subcommand)]
	pub command: Subcommands
}

#[derive(Subcommand)]
pub enum Subcommands {
	ReadPackageData {
		files: Vec<String>
	}
}

pub async fn read_package_data(db: DatabaseThing, files: Vec<String>) -> Result {
	for filename in files.into_iter() {
		let file_contents = match fs::read(&filename).await {
			Ok(file) => { file }
			Err(err) => {
				println!("error opening file {filename}: {err}");
				continue
			}
		};

		let file_str = match String::from_utf8(file_contents) {
			Ok(file) => { file }
			Err(err) => {
				println!("err parsing file {filename} as utf8: {err}");
				continue
			}
		};

		let package = serde_json::from_str::<Vec<db::NewPackage>>(&file_str);
		match package {
			Ok(packages) => {
				for package in packages {
					db.add_package(&package);
				}
			}
			Err(err) => {
				let package = serde_json::from_str::<db::NewPackage>(&file_str);
				match package {
					Ok(package) => {
						if db.contains_package(&package.name) {
							println!("package {} was not added: package with same name already addeed", package.name);
							continue
						}

						if package.repository.r#type != "git" {
							println!(
								"package {} was not added: repository does not appear to be a git repository, so it cannot be a github repo, and filing issues on non-github repositories not supported (yet?)",
								package.name
							);
							println!("filename: {filename}");
							continue
						}

						if !package.repository.url.contains("github.com") {
							println!(
								"package {} was not added: repository does not appear to be a github repository, filing issues on non-github repositories not supported (yet?)",
								package.name
							);
							println!("filename: {filename}");
							continue
						}

						let res = db.add_package(&package);
						match res {
							Ok(()) => { println!("package {} was added", package.name) }
							Err(e) => { println!("package {} was not added: {e}", package.name) }
						}
					}
					Err(err2) => { println!("package had errors!\nerr 1: {err}\nerr 2: {err2}") }
				}
			}
		}
	}

	Ok(())
}
