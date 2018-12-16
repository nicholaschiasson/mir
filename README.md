# mir

Tool to mirror a user's entire owned GitLab group hierarchy locally and
optionally clone all projects

```
mir
Nicholas Omer Chiasson <nicholasomerchiasson@gmail.com>
Tool to mirror a user's entire owned GitLab group hierarchy locally and optionally clone all projects

USAGE:
    mir [FLAGS] [OPTIONS]

FLAGS:
    -c, --clone      Clone all repositories
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --destination <destination>
            The destination directory in which the hierarchy should be mirrored [default: .]

    -H, --host <host>                                      GitLab remote host [default: gitlab.com]
    -p, --personal-access-token <personal_access_token>    GitLab personal access token
```
