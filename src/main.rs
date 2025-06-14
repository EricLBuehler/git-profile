use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser)]
#[command(name = "git-persona")]
#[command(about = "A CLI tool to manage and switch between Git user profiles")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new profile
    Add {
        /// Profile name
        name: String,
        /// Git user name
        #[arg(short, long)]
        user: String,
        /// Git email
        #[arg(short, long)]
        email: String,
    },
    /// Remove an existing profile
    Remove {
        /// Profile name to remove
        name: String,
    },
    /// List all profiles
    List,
    /// Switch to a profile
    Switch {
        /// Profile name to switch to
        name: String,
    },
    /// Show current Git user
    Current,
}

#[derive(Debug, Serialize, Deserialize)]
struct Profile {
    name: String,
    email: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Config {
    profiles: HashMap<String, Profile>,
}

impl Config {
    fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            return Ok(Config::default());
        }

        let content = fs::read_to_string(config_path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::config_path()?;
        let content = toml::to_string_pretty(self)?;

        // Create config directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(config_path, content)?;
        Ok(())
    }

    fn config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let mut path = dirs::config_dir().ok_or("Could not find config directory")?;
        path.push("git-persona");
        path.push("config.toml");
        Ok(path)
    }
}

fn git_config_set(key: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(["config", "--global", key, value])
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "Failed to set {}: {}",
            key,
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    Ok(())
}

fn git_config_get(key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(["config", "--global", key])
        .output()?;

    if !output.status.success() {
        return Ok(String::from("Not set"));
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut config = Config::load()?;

    match cli.command {
        Commands::Add { name, user, email } => {
            if config.profiles.contains_key(&name) {
                print!("Profile '{}' already exists. Overwrite? [y/N]: ", name);
                io::stdout().flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;

                if !input.trim().eq_ignore_ascii_case("y") {
                    println!("Aborted.");
                    return Ok(());
                }
            }

            config.profiles.insert(
                name.clone(),
                Profile {
                    name: user.clone(),
                    email: email.clone(),
                },
            );

            config.save()?;
            println!("✅ Profile '{}' added successfully!", name);
            println!("   Name: {}", user);
            println!("   Email: {}", email);
        }

        Commands::Remove { name } => {
            if config.profiles.remove(&name).is_some() {
                config.save()?;
                println!("✅ Profile '{}' removed successfully!", name);
            } else {
                println!("❌ Profile '{}' not found!", name);
            }
        }

        Commands::List => {
            if config.profiles.is_empty() {
                println!("No profiles configured.");
                println!(
                    "\nUse 'git-persona add <name> --user <user> --email <email>' to add a profile."
                );
            } else {
                println!("Configured profiles:");
                println!();

                let mut profiles: Vec<_> = config.profiles.iter().collect();
                profiles.sort_by_key(|(name, _)| name.as_str());

                for (profile_name, profile) in profiles {
                    println!("📋 {}", profile_name);
                    println!("   Name: {}", profile.name);
                    println!("   Email: {}", profile.email);
                    println!();
                }
            }
        }

        Commands::Switch { name } => match config.profiles.get(&name) {
            Some(profile) => {
                git_config_set("user.name", &profile.name)?;
                git_config_set("user.email", &profile.email)?;

                println!("✅ Switched to profile '{}'", name);
                println!("   Name: {}", profile.name);
                println!("   Email: {}", profile.email);
            }
            None => {
                println!("❌ Profile '{}' not found!", name);
                println!("\nAvailable profiles:");
                for profile_name in config.profiles.keys() {
                    println!("  - {}", profile_name);
                }
            }
        },

        Commands::Current => {
            let name = git_config_get("user.name")?;
            let email = git_config_get("user.email")?;

            println!("Current Git user:");
            println!("  Name: {}", name);
            println!("  Email: {}", email);

            // Check if current config matches any profile
            for (profile_name, profile) in &config.profiles {
                if profile.name == name && profile.email == email {
                    println!("\n  (Profile: {})", profile_name);
                    break;
                }
            }
        }
    }

    Ok(())
}
