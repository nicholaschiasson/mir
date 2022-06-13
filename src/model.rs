use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Group {
	pub full_path: String,
}

#[derive(Debug, Deserialize)]
pub struct Namespace {
	pub full_path: String,
}

#[derive(Debug, Deserialize)]
pub struct Project {
	pub http_url_to_repo: String,
	pub name: String,
	pub namespace: Namespace,
}

#[derive(Debug, Deserialize)]
pub struct User {
	pub username: String,
}
