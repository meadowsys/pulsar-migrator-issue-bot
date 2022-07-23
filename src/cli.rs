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
						// if package.repository.r#type != "git" {
						// 	println!(
						// 		"package {} was not added, repository is not a github repository, so cannot file issue",
						// 		package.name
						// 	);
						// 	continue
						// }
						let res = db.add_package(&package);
						if res {
							println!("package {} was added", package.name);
						} else {
							println!("package {} was not added", package.name);
						}
					}
					Err(err2) => { println!("package had errors!\nerr 1: {err}\nerr 2: {err2}") }
				}
			}
		}
	}

	Ok(())
}
