# mir

Tool to mirror a user's entire accessible GitLab group hierarchy locally and
optionally clone all projects.

## Usage

```
mir
Nicholas Omer Chiasson <nicholasomerchiasson@gmail.com>
Tool to mirror a user's entire accessible GitLab group hierarchy locally and optionally clone all
projects.

USAGE:
    mir [OPTIONS]

OPTIONS:
    -A, --access-level
            Access level of groups (and projects if --clone flag provided)
            -A     => Guest Access [default]
            -AA    => Reporter Access
            -AAA   => Developer Access
            -AAAA  => Maintainer Access
            -AAAAA => Owner Access

    -c, --clone
            Clone all repositories

    -d, --destination <DESTINATION>
            The destination directory in which the hierarchy should be mirrored [default: .]

    -h, --help
            Print help information

    -H, --host <HOST>
            GitLab remote host [default: gitlab.com]

    -p, --personal-access-token <PERSONAL_ACCESS_TOKEN>
            GitLab personal access token

    -s, --ssh-private-key <SSH_PRIVATE_KEY>
            SSH private key [default: ~/.ssh/id_rsa]

    -V, --version
            Print version information
```
