use dotenvy::dotenv;
use hyper::body::Body;
use hyper::body::HttpBody as _;
use hyper::{ Method, Request };
use pulsar_migrator_issue_bot::http::get_http_client;
use pulsar_migrator_issue_bot::db;
use pulsar_migrator_issue_bot::Result;
use std::env::var;

#[tokio::main]
async fn main() -> Result {
	let db_filename = "state.ron";

	let _ = dotenv();

	let token = var("GITHUB_TOKEN")
		.map_err(|e| format!("error in fetching GITHUB_TOKEN: {}", e))?;

	let db = db::DatabaseThing::new("state.ron", 60).await?;

	// let client = get_http_client();

	// let request = Request::builder()
	// 	.uri("https://api.github.com/zen")
	// 	.header("user-agent", "autumnblazey/pulsar-migrator-issue-bot")
	// 	.method(Method::GET)
	// 	.body(Body::empty())?;

	// let mut zen = client.request(request).await?;
	// let body = zen.body_mut();
	// let mut zen = String::new();

	// while let Some(chunk) = body.data().await {
	// 	zen.push_str(&String::from_utf8(chunk?.to_vec())?);
	// }

	// println!("{}", zen);

	Ok(())
}
