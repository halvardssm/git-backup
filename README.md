# Git Backup Script

> This is a script for personal usage. That means that breaking changes can occur at any time. If someone finds this script usefull and want to use it, please make a fork, or consider creating an issue so that I can version changes properly. If you can see this message means that noone has created an issue, and I assume that I am the only one using it.

This is a script to pull/backup a git repo from an interval.

This script needs a config file to work. Create a yaml file and pass it with the arg `--config`, e.g. `--config=../repo` or `--config=/path/repo`.

The schema of the config file is as follows:

```yaml
interval: u64 # Default interval (in seconds), will be used if interval for repo is not set
repos: # Array of repos
    - path: str # Repo path
      interval: u64? # Optional interval (in seconds)
```

Example:

```yaml
interval: 10
repos:
    - path: "../repo"
      interval: 15
    - path: "/path/repo"
```
