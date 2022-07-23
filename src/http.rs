//! hyper http client

use hyper::{ Client, client::HttpConnector };
use hyper_rustls::{ HttpsConnector, HttpsConnectorBuilder };

lazy_static! {
	static ref CLIENT: Client<HttpsConnector<HttpConnector>, hyper::Body> = {
		Client::builder()
			.http2_only(true)
			.build::<_, hyper::Body>({
				HttpsConnectorBuilder::new()
					.with_native_roots()
					.https_only()
					.enable_http2()
					.build()
			})
	};
}

#[inline]
pub fn get_http_client() -> Client<HttpsConnector<HttpConnector>, hyper::Body> {
	CLIENT.clone()
}
