# Git Backup Script

> This is a script for personal usage. That means that breaking changes can occur at any time. If someone finds this script usefull and want to use it, please make a fork, or consider creating an issue so that I can version changes properly. If you can see this message means that noone has created an issue, and I assume that I am the only one using it.

This is a script to backup a git repo, and update it according to the given interval.

It uses `git clone --mirror` for the initial cloning, and then `git update` for subsequent pulls

This script needs a config file to work. Create a yaml file and pass it with the arg `--config`, e.g. `--config=../repo` or `--config=/path/repo`.

The schema of the config file is as follows:

## Examples

### Config

- `auth_token` is optional and is only used when you want to query private repos

```yaml
interval: 10
path: /mnt/storage/git-backup
repos:
  - url: "git@gitlab.com:halvardm/rust-gitlab.git"
owners:
  - provider: "github_user"
    namespace: "gituser1"
    auth_token: "ghp_xxxxx"
  - provider: "github_org"
    namespace: "gitorg1"
    auth_token: "ghp_xxxxx"
  - provider: "gitlab_group"
    namespace: "gitgroup1"
    auth_token: "glpat-xxxxx"
  - provider: "gitlab_user"
    namespace: "gituser2"
    auth_token: "glpat-xxxxx"
```

Will result in the following file structure

```
.
├── github.com
│   ├── gituser1
│   │   └── some-repo.git
│   └── gitorg1
│       ...
│       └── some-other-repo.git
└── gitlab.com
    ├── gituser2
    │   └── some-third-repo.git
    └── gitgroup1
        ├── gitsubgroup1
        │   ├── gitsubsubgroup1
        │   │   └── some-third-level-repo.git
        │   └── some-second-level-repo.git
        └── some-top-level-repo.git
```

### SSH

You will need to have the SSH key added to the remote where you want to clone from, and to authorize remotes for your local environment.

To authorize github and gitlab, you can use these scripts, however this is not recommended, and could be considered a security issue.

```shell
ssh-keyscan github.com >> ~/.ssh/known_hosts
ssh-keyscan gitlab.com >> ~/.ssh/known_hosts
```

### Systemd

If you use systemd, you can use this template

```
[Unit]
Description=Git backup service
After=network-online.target

[Service]
Type=exec
ExecStart=/bin/git-backup --config=/path/to/config/config.yaml

[Install]
WantedBy=network-online.target
```
