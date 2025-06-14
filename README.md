# git-persona

**git-persona** is a command-line tool to manage and switch between multiple Git user profiles easily.

This tool is useful if you need to maintain multiple Git identities (e.g. work and personal) and want to quickly switch your global user.name and user.email settings.

---

## Features
- Add, remove, and list named Git user profiles.
- Switch your global git user settings with a single command.
- View which profile is currently active.

## Install

### Cargo

Install from crates.io:

```
cargo install git-persona
```

Or build and run from the repo:

```
cargo install --path .
./target/release/git-persona --help
```

---

## Usage

### Add a profile

```
git-persona add <name> --user <user.name> --email <user.email>
# Example:
git-persona add work --user "Work Name" --email work@example.com
```

### List all profiles

```
git-persona list
```

### Switch to a profile

```
git-persona switch <name>
# Example:
git-persona switch work
```

### View current user

```
git-persona current
```

### Remove a profile

```
git-persona remove <name>
```

---

## Where are profiles stored?

Profiles are stored in a config file at:
- Linux/macOS: `~/.config/git-persona/config.toml`

No information is ever sent anywhere; the config is local only.

---

## License

MIT © 2025 Eric Buehler