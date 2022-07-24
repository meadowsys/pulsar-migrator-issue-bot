//! github client used to access github's api

use octocrab::Octocrab;
use std::sync::Mutex;

pub struct GithubClient {
	octocrab: Octocrab
}

impl GithubClient {
	pub fn new(token: &str) -> crate::Result<Self> {
		let octocrab = Octocrab::builder()
			.personal_token(token.into())
			.build()?;

		Ok(Self { octocrab })
	}
}
