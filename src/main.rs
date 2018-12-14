use gitlab::Gitlab;
use rpassword;
use structopt::StructOpt;

use std::collections::HashSet;
use std::fs;

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
    let password = rpassword::prompt_password_stdout("Enter GitLab personal access token: ")?;
    let gitlab = Gitlab::new(&args.host, password)?;
    let projects = gitlab.owned_projects()?;
    projects.iter().map(|ref p| &p.namespace.full_path).collect::<HashSet<_>>().iter().for_each(|n| {
        let namespace = format!("{}/{}", args.destination, n);
        fs::create_dir_all(&namespace).expect(&format!("failed to create the directory '{}'", n));
    });
    Ok(())
}
