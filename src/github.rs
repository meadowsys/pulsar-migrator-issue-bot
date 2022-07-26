//! github client used to access github's api

use crate::db::DatabaseThing;
use octocrab::Octocrab;
use std::sync::Mutex;

lazy_static! {
	static ref ISSUE_CONTENT: (String, String) = {
		let plain = include_str!("../resources/issue_template.md");
		let (title, body) = plain.split_once("\n\n").unwrap();
		(title.into(), body.into())
	};
}

pub struct GithubClient {
	octocrab: Octocrab
}

impl GithubClient {
	pub fn new(token: &str, db: DatabaseThing) -> crate::Result<Self> {
		let octocrab = Octocrab::builder()
			.personal_token(token.into())
			.build()?;

		Ok(Self { octocrab })
	}

	pub async fn create_permission_request_issue(
		&self,
		owner: &str,
		repo: &str
	) -> crate::Result<String> {
		let (title, body) = &*ISSUE_CONTENT;

		let req = self.octocrab.issues(owner, repo)
			.create(title)
			.body(body)
			.send().await?;

		Ok(req.url.to_string())
	}
}
