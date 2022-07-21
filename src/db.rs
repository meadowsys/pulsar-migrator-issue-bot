use serde::{ Deserialize, Serialize };
use std::fs as sync_fs;
use std::path;
use std::sync::{ Arc, Mutex, MutexGuard };
use std::time::SystemTime;
use tokio::fs as async_fs;
use tokio::spawn;

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
	pub last_write_call_time: SystemTime,
	pub debounce_time: u64
}

#[derive(Clone, Deserialize, Serialize)]
struct DatabaseThingData {
	last_updated: SystemTime
}

impl DatabaseThing {
	pub async fn new(filename: &str, debounce_time: u64) -> crate::Result<Self> {
		let data = if path::Path::new(filename).exists() {
			let data = async_fs::read(filename).await
				.map_err(|e| format!("error reading file {filename}: {e}"))?;

			let data = String::from_utf8(data)
				.map_err(|e| format!("error parsing text in file {filename}: {e}"))?;

			ron::from_str(&data)
				.map_err(|e| format!("error parsing ron in file {filename}: {e}"))?
		} else {
			let data = DatabaseThingData {
				last_updated: SystemTime::now()
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
					last_write_call_time: SystemTime::now(),
					debounce_time
				},
				data
			}))
		};

		Ok(new)
	}

	/// async because then we have to `await` it, which means it has to be called  within
	/// a runtime, which is likely tokio because we don't have any other dependencies that
	/// provide other types of runtimes,then we can safely call `tokio::spawn`
	async fn write_to_file(&self) {
		let cloned = self.clone();
		spawn(async move {
			let _ = write_to_file_inner(cloned).await;
		});

		async fn write_to_file_inner(db: DatabaseThing) -> crate::Result {
			#[inline]
			fn get_vals(db: &DatabaseThing) -> (SystemTime, u64) {
				let mut inner = db.lock_inner();
				let last_write_call_time = inner.meta.last_write_call_time;
				let debounce_time = inner.meta.debounce_time;
				inner.meta.last_write_call_time = SystemTime::now();
				(last_write_call_time, debounce_time)
			}

			let (last_write_call_time, debounce_time) = get_vals(&db);
			if last_write_call_time.elapsed()?.as_secs() < debounce_time { return Ok(()) }

			db.write_to_file_immediately();
			Ok(())
		}
	}

	fn write_to_file_immediately(&self) {
		fn write_to_file_immediately_inner(db: &DatabaseThing) -> crate::Result {
			let mut inner = db.lock_inner();

			inner.data.last_updated = SystemTime::now();
			let data = ron::ser::to_string_pretty(&inner.data, DatabaseThing::pretty_config())?;
			let filename = inner.meta.filename.clone();
			drop(inner);

			sync_fs::write(&filename, &data)?;
			Ok(())
		}

		let res = write_to_file_immediately_inner(self);
		if let Err(e) = res {
			eprintln!("error when writing database file: {e}");
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
		self.write_to_file_immediately();
	}
}
