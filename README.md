# Git Backup Script

> This is a script for personal usage. That means that breaking changes can occur at any time. If someone finds this script usefull and want to use it, please make a fork, or consider creating an issue so that I can version changes properly. If you can see this message means that noone has created an issue, and I assume that I am the only one using it.

This is a script to backup a git repo, and update it according to the given interval.

It uses `git clone --mirror` for the initial cloning, and then `git update` for subsequent pulls

This script needs a config file to work. Create a yaml file and pass it with the arg `--config`, e.g. `--config=../repo` or `--config=/path/repo`.

The schema of the config file is as follows:

## Examples

### Config

```yaml
interval: 10
path: /mnt/storage/git-backup
repos:
    - url: "git@github.com:halvardssm/js-helpers.git" # will get downloaded to a sub folder named `individual`
owners:
   - provider: "github_user"
     namespace: "halvardssm"
   - provider: "github_org"
     namespace: "halvardorg"
   - provider: "gitlab_org"
     namespace: "halvardorg"
   - provider: "gitlab_user"
     namespace: "halvardm"
```

Will result in the following file structure

```
.
├── github_org
│   └── halvardorg
│       ...
│       └── story-book.git
├── github_user
│   └── halvardssm
│       ...
│       └── git-backup.git
│── gitlab_org
│   └── gitlab.com
│       └── halvardorg
└── gitlab_user
    └── gitlab.com
        └── halvardm
```

### Systemd

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

## Todo

- Add authentication for private repos
- improve folder structure