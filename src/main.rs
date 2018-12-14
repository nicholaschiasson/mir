use gitlab::Gitlab;
use rpassword::prompt_password_stdout;
use structopt::StructOpt;

use std::fs::create_dir_all;

/// A tool to mirror a GitLab user's group hierarchy on the local file system
#[derive(StructOpt, Debug)]
#[structopt(name = "mir")]
struct CliArgs {
    /// Clone all repositories
    #[structopt(short = "c", long = "clone")]
    clone: bool,
    /// The destination directory in which the hierarchy should be mirrored
    #[structopt(short = "d", long = "destination", default_value = ".")]
    destination: String,
    /// GitLab remote host
    #[structopt(short = "H", long = "host", default_value = "gitlab.com")]
    host: String,
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: u8,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::from_args();
    let password = prompt_password_stdout("Enter GitLab personal access token: ")?;
    let gitlab = Gitlab::new(&args.host, password)?;
    let projects = gitlab.owned_projects()?;
    let mut namespaces = projects.iter().map(|ref p| &p.namespace.full_path).collect::<Vec<_>>();
    namespaces.dedup();
    namespaces.iter().for_each(|n| {
        let namespace = format!("{}/{}", args.destination, n);
        create_dir_all(&namespace).expect(&format!("failed to create the directory '{}'", n));
    });
    Ok(())
}
