use serde::{ Deserialize, Serialize };
use std::fs as sync_fs;
use std::path;
use std::sync::{ Arc, Mutex, MutexGuard };
use std::time::SystemTime;
use tokio::fs as async_fs;

/// cheapo database-ish sort of file to store state
#[derive(Clone)]
pub struct DatabaseThing {
	inner: Arc<Mutex<DatabaseThingInner>>
}

struct DatabaseThingInner {
	pub meta: DatabaseThingMeta,
	pub data: DatabaseThingData
}

struct DatabaseThingMeta {
	pub filename: String,
	pub last_write_call_time: SystemTime
}

#[derive(Clone, Deserialize, Serialize)]
struct DatabaseThingData {
	pub last_updated: SystemTime,
	pub packages: Vec<PackageState>
}

#[derive(Clone, Deserialize, Serialize)]
enum PackageState {
	New(NewPackage)
}

#[derive(Clone, Deserialize, Serialize)]
pub struct NewPackage {
	pub name: String,
	pub repository: PackageRepository,
	pub downloads: u32,
	pub stargazers_count: u32
}

#[derive(Clone, Deserialize, Serialize)]
pub struct PackageRepository {
	pub r#type: String,
	pub url: String
}

impl DatabaseThing {
	pub async fn new(filename: &str) -> crate::Result<Self> {
		let data = if path::Path::new(filename).exists() {
			let data = async_fs::read(filename).await
				.map_err(|e| format!("error reading file {filename}: {e}"))?;

			let data = String::from_utf8(data)
				.map_err(|e| format!("error parsing text in file {filename}: {e}"))?;

			ron::from_str(&data)
				.map_err(|e| format!("error parsing ron in file {filename}: {e}"))?
		} else {
			let data = DatabaseThingData {
				last_updated: SystemTime::now(),
				packages: vec![]
			};
			// let ser_data = ron::to_string(&data)?;
			let ser_data = ron::ser::to_string_pretty(&data, Self::pretty_config())?;
			async_fs::write(filename, ser_data).await?;
			data
		};

		let new = Self {
			inner: Arc::new(Mutex::new(DatabaseThingInner {
				meta: DatabaseThingMeta {
					filename: filename.into(),
					last_write_call_time: SystemTime::now()
				},
				data
			}))
		};

		Ok(new)
	}

	pub fn add_package(&self, package: &NewPackage) -> bool {
		if self.package_exists(&package.name) { return false }

		let mut inner = self.lock_inner();
		inner.data.packages.push(PackageState::New(package.clone()));

		true
	}

	pub fn package_exists(&self, package_name: &str) -> bool {
		let inner = self.lock_inner();

		for package in inner.data.packages.iter() {
			let res = match package {
				PackageState::New(NewPackage { name, .. }) => { name == package_name }
			};
			if res { return true }
		}

		false
	}

	// async fn write_to_file(&self) {
	// 	let _ = write_to_file_inner(self).await;

	// 	async fn write_to_file_inner(db: &DatabaseThing) -> crate::Result {
	// 		#[inline]
	// 		fn get_vals(db: &DatabaseThing) -> (SystemTime, u64) {
	// 			let inner = db.lock_inner();
	// 			let last_write_call_time = inner.meta.last_write_call_time;
	// 			let throttle_time = inner.meta.throttle_time;
	// 			(last_write_call_time, throttle_time)
	// 		}

	// 		let (last_write_call_time, throttle_time) = get_vals(db);
	// 		if last_write_call_time.elapsed()?.as_secs() > throttle_time { return Ok(()) }

	// 		db.write_to_file_immediately();
	// 		Ok(())
	// 	}
	// }

	fn write_to_file_immediately(&self) {
		fn write_to_file_immediately_inner(db: &DatabaseThing) -> crate::Result {
			let mut inner = db.lock_inner();

			let now = SystemTime::now();
			inner.meta.last_write_call_time = now;
			inner.data.last_updated = now;

			let data = ron::ser::to_string_pretty(&inner.data, DatabaseThing::pretty_config())?;
			let filename = inner.meta.filename.clone();
			drop(inner);

			sync_fs::write(&filename, &data)?;
			Ok(())
		}

		let res = write_to_file_immediately_inner(self);
		if let Err(e) = res {
			println!("error when writing database file: {e}");
		}
	}

	fn lock_inner(&self) -> MutexGuard<'_, DatabaseThingInner> {
		match self.inner.lock() {
			Ok(lock) => { lock }
			Err(e) => { e.into_inner() }
		}
	}

	fn pretty_config() -> ron::ser::PrettyConfig {
		ron::ser::PrettyConfig::new()
			.new_line("\n".into())
			.indentor("\t".into())
			.struct_names(true)
	}
}

impl Drop for DatabaseThing {
	fn drop(&mut self) {
		let inner = self.lock_inner();
		println!(
			"db stats:\n   total packages: {}",
			inner.data.packages.len()
		);
		// otherwise we deadlock on the call to `self.write_to_file_immediately();`
		drop(inner);

		self.write_to_file_immediately();
	}
}
