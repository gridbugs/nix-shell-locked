use anyhow::Result;
use serde::Deserialize;
use serde_json::Value;
use std::{fs, path::PathBuf, process};
use xdg::BaseDirectories;

mod cli;
mod defaults;

use cli::{Args, Override};

#[derive(Deserialize)]
struct Config {
    flake_lockfile: String,
}

impl Config {
    fn expand_flake_lockfile_path(&self) -> Result<PathBuf> {
        Ok(shellexpand::full(self.flake_lockfile.as_str())?
            .to_string()
            .into())
    }
}

const CONFIG_FILE_SUGGESTION: &str = "Try creating a file at ~/.config/nix-shell-locked.toml with contents:\n\nflake_lockfile = \"path/to/flake.lock\"";

fn get_config_file_path(config_file_path: Option<PathBuf>) -> Result<PathBuf> {
    let config_file_path = if let Some(config_file_path) = config_file_path {
        if config_file_path.is_file() {
            config_file_path
        } else {
            anyhow::bail!(
                "Specified config file ({}) does not exist.",
                config_file_path.display()
            );
        }
    } else {
        let base_directories = BaseDirectories::new()?;
        if let Some(config_file_path) = base_directories.find_config_file(defaults::CONFIG_FILENAME)
        {
            config_file_path
        } else {
            eprintln!("Can't find config file!\n{}", CONFIG_FILE_SUGGESTION);
            process::exit(1);
        }
    };
    Ok(config_file_path)
}
fn read_config(config_file_path: Option<PathBuf>) -> Result<Config> {
    let config_file_path = get_config_file_path(config_file_path)?;
    log::info!("using config file: {}", config_file_path.display());
    let config_file_text = fs::read_to_string(config_file_path)?;
    Ok(toml::from_str(config_file_text.as_str())?)
}

fn get_config(override_: Option<Override>) -> Result<Config> {
    let config = match override_ {
        None => read_config(None)?,
        Some(Override::ConfigFile(config_file_path)) => read_config(Some(config_file_path))?,
        Some(Override::FlakeLockfile(flake_lockfile)) => Config { flake_lockfile },
    };
    Ok(config)
}

fn read_nixpkgs_revision(flake_lockfile_path: PathBuf) -> Result<String> {
    let flake_lockfile_text = fs::read_to_string(&flake_lockfile_path)?;
    let flake_lockfile_json = flake_lockfile_text.parse::<Value>()?;
    let nixpkgs_revision_path = "/nodes/nixpkgs/locked/rev";
    let revision = flake_lockfile_json
        .pointer(nixpkgs_revision_path)
        .ok_or(anyhow::anyhow!(
            "Couldn't find path \"{}\" in flake lockfile {}",
            nixpkgs_revision_path,
            flake_lockfile_path.display()
        ))?
        .as_str()
        .ok_or(anyhow::anyhow!(
            "The value at \"{}\" in {} is not a string.",
            nixpkgs_revision_path,
            flake_lockfile_path.display()
        ))?
        .to_string();
    Ok(revision)
}

fn nixpkgs_flake_installable_name(package_name: &str, nixpkgs_revision: &str) -> String {
    format!("nixpkgs/{}#{}", nixpkgs_revision, package_name)
}

fn make_command(
    packages: &[String],
    nixpkgs_revision: &str,
    passthrough_args: &[String],
) -> Vec<String> {
    let mut args = vec!["nix".to_string(), "shell".to_string()];
    args.extend(
        packages
            .into_iter()
            .map(|package| nixpkgs_flake_installable_name(package.as_str(), nixpkgs_revision)),
    );
    args.extend(passthrough_args.into_iter().cloned());
    args
}

fn run() -> Result<()> {
    let Args {
        dryrun,
        override_,
        packages,
        passthrough_args,
    } = Args::parse();
    let config = get_config(override_)?;
    log::info!("using lockfile: {}", config.flake_lockfile);
    let nixpkgs_revision = read_nixpkgs_revision(config.expand_flake_lockfile_path()?)?;
    log::info!("nixpkgs revision is: {}", nixpkgs_revision);
    let command = make_command(&packages, &nixpkgs_revision, &passthrough_args);
    if dryrun {
        println!("{}", command.join(" "));
        Ok(())
    } else {
        let error = exec::execvp("nix", &command);
        eprintln!("{}", error);
        process::exit(1);
    }
}

fn main() {
    env_logger::init();
    if let Err(e) = run() {
        log::error!("{}", e);
        process::exit(1);
    }
}
