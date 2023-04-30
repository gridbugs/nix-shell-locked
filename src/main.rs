use serde::Deserialize;
use serde_json::Value;
use std::{fs, path::PathBuf, process};
use xdg::BaseDirectories;

mod cli;
mod defaults;

use cli::{Args, Override};

fn get_config_file_path(config_file_path: Option<PathBuf>) -> PathBuf {
    if let Some(config_file_path) = config_file_path {
        if config_file_path.is_file() {
            config_file_path
        } else {
            panic!("user provided path doesn't exist");
        }
    } else {
        let base_directories = BaseDirectories::new().unwrap();
        if let Some(config_file_path) = base_directories.find_config_file(defaults::CONFIG_FILENAME)
        {
            config_file_path
        } else {
            panic!("can't find config file");
        }
    }
}

#[derive(Deserialize)]
struct Config {
    flake_lockfile: String,
}

impl Config {
    fn expand_flake_lockfile_path(&self) -> PathBuf {
        shellexpand::full(self.flake_lockfile.as_str())
            .unwrap()
            .to_string()
            .into()
    }
}

fn read_config(config_file_path: Option<PathBuf>) -> Config {
    let config_file_path = get_config_file_path(config_file_path);
    log::info!("using config file: {}", config_file_path.display());
    let config_file_text = fs::read_to_string(config_file_path).unwrap();
    toml::from_str(config_file_text.as_str()).unwrap()
}

fn read_nixpkgs_revision(flake_lockfile_path: PathBuf) -> String {
    let flake_lockfile_text = fs::read_to_string(flake_lockfile_path).unwrap();
    let flake_lockfile_json = flake_lockfile_text.parse::<Value>().unwrap();
    flake_lockfile_json
        .pointer("/nodes/nixpkgs/locked/rev")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string()
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

fn main() {
    env_logger::init();
    let Args {
        dryrun,
        override_,
        packages,
        passthrough_args,
    } = Args::parse();
    let config = match override_ {
        None => read_config(None),
        Some(Override::ConfigFile(config_file_path)) => read_config(Some(config_file_path)),
        Some(Override::FlakeLockfile(flake_lockfile)) => Config { flake_lockfile },
    };
    log::info!("using lockfile: {}", config.flake_lockfile);
    let nixpkgs_revision = read_nixpkgs_revision(config.expand_flake_lockfile_path());
    log::info!("nixpkgs revision is: {}", nixpkgs_revision);
    let command = make_command(&packages, &nixpkgs_revision, &passthrough_args);
    if dryrun {
        println!("{}", command.join(" "));
    } else {
        let error = exec::execvp("nix", &command);
        eprintln!("{}", error);
        process::exit(1);
    }
}
