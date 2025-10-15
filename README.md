# Bitwarden krunner plugin using the `rbw` bitwarden CLI

This is a krunner plugin for KDE Plasma 6 that uses the unofficial `rbw` bitwarden CLI.
It searches for matching entries and puts them into the clipboard.

I've developed this for personal use, but feel free to ask me if you need help setting
it up or want to use this repo as a template for your own plugin.

## Features

- Search for entries and copy the password to the clipboard
  - by default, you will need to use the prefix `"pw "`
  - the password will be copied to the clipboard when pressing `enter`
- Show all info for an entry in a notification by pressing `shift + enter`
- Due to the awesome `rbw` CLI, you will be promoted for your password if
  the local database is locked
- `rbw` also makes searching fast, since everything is happening locally
- Quick command for manually syncing the database (default `pws`)

## Requirements

- you will need to use KDE Plasma 6 
- install and set up [rbw](https://github.com/doy/rbw)
- `cargo` for building

## Installation

As of right now, this plugin is not submitted to the KDE plugin store, so you need to install it manually.

- clone this repo with `git`
- run `./install.sh` inside

A quick overview what the `install.sh` script does:
- `cargo build`
- copy the `.desktop` file to `~/.local/share/krunner/dbusplugins/`
  - this informs `krunner` about the existence of this plugin
- copy the `.service` file to `~/.local/share/dbus-1/services/`
  - this sets up a [DBus activatable service](https://dbus.freedesktop.org/doc/dbus-specification.html#message-bus-starting-services)
  - when `krunner` wants to communicate with the plugin, DBus will detect that
    if the service is not running and automatically start it
- create a config file at `~/.config/bitwarden-rbw-krunner/config.toml`
  - only if it does not exist yet
- "restart" krunner and the plugin itself
  - technically it is just stopping both, but they are started automatically again

## Configuration

The plugin is configured using a config file at `~/.config/bitwarden-rbw-krunner/config.toml`.
The `install.sh` script will create a default config.

See `config.example.toml` for available options.

Note that changes to the config will not be applied until you restart the plugin:

```bash
pkill -f bitwarden-rbw-krunner
```

## Attributions

This plugin very much depends on the [`krunner`](https://crates.io/crates/krunner) crate,
which handels all the DBus stuff.

I also looked at some python templates for inspiration:

- https://github.com/alex1701c/KDevelopTemplates/tree/master
- https://github.com/jimcornmell/scriptRunner/tree/main

## Development

If you are not making a lot of change, you can just use the `install.sh` script
everytime you want to test your changes. But this can get tedious for more complex
development. A better way to develop is by setting up the krunner plugin, but not
installing the dbus service. For that, uninstall if you had it installed
before:

```bash
./uninstall.sh
```

Next, manually set up the plugin:

```bash
mkdir -p ~/.local/share/krunner/dbusplugins/
cp bitwarden-rbw-krunner.desktop ~/.local/share/krunner/dbusplugins/
```

Now, you can run the plugin with `cargo run`.

If you made changes to the `.desktop` file, you probably need to restart `krunner`:

```bash
kquitapp6 krunner
```

## Contributing

Contributions are generally welcome, as long as they don't have a negative
effect on any current feature. Feel free to create an issue or e-mail me
if you are unsure, have questions or need help.

## General krunner plugin info

More information can be found here:  
https://invent.kde.org/frameworks/krunner/-/blob/master/src/data/org.kde.krunner1.xml  
https://develop.kde.org/docs/use/d-bus/introduction_to_dbus/
