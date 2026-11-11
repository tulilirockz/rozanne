<div align="center">
    <img width="120" height="106" alt="rozanne" src="https://github.com/user-attachments/assets/1c740fb6-5e78-448a-a578-5734a9dc60fd" />
    <br/>
    <h1>Rozanne</h1>
</div>
<br/>

Modular updater tool for operating systems

## Installation

Not yet packaged anywhere, will be on Terra at some point

## Utilization

Rozanne is composed of two services that do privileged/unprivileged updates respectively

```
systemctl enable --now rozanne
systemctl enable --now --user rozanne
```

Those would run all of the updates related to their privilege level, root user run `system` modules, other users run the `user` level.

Module definitions have the following shape:
```toml
[main]
name = "flatpak"
description = "Updates flatpaks"
context = "user" # This will run for any user that rozanne is running as, not any target user
program = "flatpak"
args = [ "update", "--yes", "--user" ]

[notify]
# Runs on module failure if verbose mode is enabled
program = "notify-send"
args = [ "-u", "critical", "-a", "rozanne-flatpak-module", "-i", "flatpak", "-n", "flatpak", "Failure running flatpak module", "Details for the failure here idk"   ]

[check]
program = "/usr/libexec/flatpak-checker" # A program that will check if the module can be ran at all, given exit code 1 it'll skip the module + show a notification if verbose
# args = [ "" ]
```

These follow the XDG standard paths for where they can be written to, `/usr/share/rozanne/modules.d`, `/etc/rozanne/modules.d`, `$HOME/.config/rozanne/modules.d` and `$XDG_CONFIG_PATH/rozanne/modules.d`, same with the `config.toml` file, which can be defined similarly to this:
```toml
# These two are self-exclusionary
enabled_modules = ["flatpak"] # Allowlist pattern
disabled_modules = [ "bootc" ] # Denylist pattern
log_level = "debug" # Follows `RUST_LOG` log levels from `log` + `env_log` crates. (https://docs.rs/log/latest/log/enum.Level.html)
verbose_notifications = false # Allows every module to notify failures 
notify_on_failure = true # Displays a notification at the end of all modules that failed running
notify_on_start = true # Displays a notification that updates are happening when this is enabled
ignore_failures = [ "brew" ] # Modules to ignore failures on (no notifications, checks proceed even if they fail) 
```

Rozanne will then run every module enabled in alphabetical sequence.

Need a blogpost :3
