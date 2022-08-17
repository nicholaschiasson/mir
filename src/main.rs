use clap::Parser;
use dirs;
use git2::{build::RepoBuilder, Cred, Error, FetchOptions, RemoteCallbacks};
use gitlab::api::common::AccessLevel;
use gitlab::api::{self, groups, projects, users, Pagination, Query};
use gitlab::Gitlab;
use indicatif::{ProgressBar, ProgressStyle};
use rpassword;

use std::collections::HashSet;
use std::fs;
use std::path::Path;

mod model;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
	#[clap(
		short = 'A',
		long,
		parse(from_occurrences),
		takes_value = false,
		help = "Access level of groups (and projects if --clone flag provided)
-A     => Guest Access [default]
-AA    => Reporter Access
-AAA   => Developer Access
-AAAA  => Maintainer Access
-AAAAA => Owner Access"
	)]
	access_level: u8,
	/// Clone all repositories
	#[clap(short, long)]
	clone: bool,
	/// The destination directory in which the hierarchy should be mirrored
	#[clap(short, long, default_value = ".")]
	destination: String,
	/// GitLab remote host
	#[clap(short = 'H', long, default_value = "gitlab.com")]
	host: String,
	/// GitLab personal access token
	#[clap(short, long)]
	personal_access_token: Option<String>,
	/// SSH private key
	#[clap(short, long, default_value = "~/.ssh/id_rsa")]
	ssh_private_key: String,
	// /// Verbose mode (-v, -vv, -vvv, etc.)
	// #[clap(short, long, parse(from_occurrences))]
	// verbose: u8,
}

fn into_access_level(access_level: u8) -> Result<AccessLevel, Box<dyn std::error::Error>> {
	match access_level.clamp(1, 5) {
		1 => Ok(AccessLevel::Guest),
		2 => Ok(AccessLevel::Reporter),
		3 => Ok(AccessLevel::Developer),
		4 => Ok(AccessLevel::Maintainer),
		5 => Ok(AccessLevel::Owner),
		// This error should be impossible to trigger because we clamp the access level
		a => Err(format!("Casting AccessLevel from invalid number {}.", a).into()),
	}
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let args = Args::parse();
	let access_level: AccessLevel = into_access_level(args.access_level)?;
	let password = match args.personal_access_token {
		Some(p) => p,
		None => rpassword::prompt_password("Enter GitLab personal access token: ")?,
	};
	let client = Gitlab::new(&args.host, &password)?;
	let ret_user: model::User = users::CurrentUser::builder().build()?.query(&client)?;
	let ret_groups: Vec<model::Group> = api::paged(
		groups::Groups::builder()
			.min_access_level(access_level)
			.build()?,
		Pagination::All,
	)
	.query(&client)?;
	let ret_projects: Vec<model::Project> = api::paged(
		projects::Projects::builder()
			.min_access_level(access_level)
			.build()?,
		Pagination::All,
	)
	.query(&client)?;
	let mut namespaces = ret_groups
		.iter()
		.map(|ref g| &g.full_path)
		.collect::<HashSet<_>>();
	namespaces.insert(&ret_user.username);
	for n in namespaces {
		let namespace = format!("{}/{}", args.destination, n);
		println!("mkdir '{}'", namespace);
		fs::create_dir_all(&namespace)?;
	}
	if args.clone {
		let progress_bar = ProgressBar::new(0);
		progress_bar.set_style(
			ProgressStyle::default_bar()
				.template("[{elapsed_precise}] {msg:18}: {bar:40.magenta/red} {pos}/{len}")
				.progress_chars("##-"),
		);
		let mut remote_callbacks = RemoteCallbacks::new();
		let ssh_private_key = args.ssh_private_key;
		remote_callbacks
			.credentials(move |_, username_from_url, allowed_types| {
				if allowed_types.is_user_pass_plaintext() {
					return Cred::userpass_plaintext(&ret_user.username, &password);
				}

				if allowed_types.is_ssh_key() {
					let mut private_key = Path::new(&ssh_private_key).to_path_buf();
					if let Ok(suffix) = private_key.strip_prefix("~") {
						private_key = dirs::home_dir()
							.ok_or_else(|| {
								Error::from_str("Can't find home directory to locate SSH private key")
							})?
							.join(suffix);
					}

					return Cred::ssh_key(
						username_from_url.ok_or_else(|| Error::from_str("No username in URL for SSH"))?,
						None,
						&private_key,
						None,
					);
				}

				Err(Error::from_str(&format!(
					"Unsupported requested credential type '{:?}'",
					allowed_types
				)))
			})
			.transfer_progress(|p| {
				if p.received_objects() < p.total_objects() {
					progress_bar.set_message("Receiving objects");
					progress_bar.set_length(p.total_objects() as u64);
					progress_bar.set_position(p.received_objects() as u64);
				} else {
					progress_bar.set_message("Resolving deltas");
					progress_bar.set_length(p.total_deltas() as u64);
					progress_bar.set_position(p.indexed_deltas() as u64);
				}
				true
			});
		let mut fetch_options = FetchOptions::new();
		fetch_options.remote_callbacks(remote_callbacks);
		let mut repo_builder = RepoBuilder::new();
		repo_builder.fetch_options(fetch_options);
		for p in ret_projects {
			let namespace = format!("{}/{}/{}", args.destination, p.namespace.full_path, p.name);
			fs::create_dir_all(&namespace)
				.expect(&format!("failed to create the directory '{}'", namespace));
			match repo_builder.clone(&p.http_url_to_repo, Path::new(&namespace)) {
				Ok(_) => progress_bar.println(format!("Cloning into '{}'...", namespace)),
				Err(error) => {
					progress_bar.println(format!("Failed to clone into '{}': {}\n", namespace, error))
				},
			}
			progress_bar.finish();
		}
		progress_bar.finish_and_clear();
	}
	Ok(())
}
