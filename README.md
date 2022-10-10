# Git Backup Script

> This is a script for personal usage. That means that breaking changes can occur at any time. If someone finds this script usefull and want to use it, please make a fork, or consider creating an issue so that I can version changes properly. If you can see this message means that noone has created an issue, and I assume that I am the only one using it.

This is a script to pull/backup a git repo from an interval.

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
     namespace: "simplyundoable"
   - provider: "gitlab_org"
     namespace: "simplyundoable"
   - provider: "gitlab_user"
     namespace: "halvardm"
```

### Systemd

```toml
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