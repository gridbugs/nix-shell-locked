use std::{path::PathBuf, process};

struct CliArgs {
    dryrun: bool,
    override_config_file: Option<PathBuf>,
    override_flake_lockfile: Option<String>,
    packages: Vec<String>,
    passthrough_args: Vec<String>,
}

impl CliArgs {
    fn parser() -> impl meap::Parser<Item = Self> {
        meap::let_map! {
            let {
                dryrun =
                    flag("dryrun")
                        .desc("print the command that would be executed instead of executing it");
                override_config_file =
                    opt_opt("PATH", "config")
                        .name('c')
                        .desc(format!(
                            "path to config file to use (defaults to $XDG_CONFIG_HOME/{})",
                            crate::defaults::CONFIG_FILENAME,
                        ));
                override_flake_lockfile =
                    opt_opt("PATH", "lockfile")
                        .name('l')
                        .desc("path to flake lockfile to use when determining nixpkgs revision");
                packages =
                    pos_multi("PACKAGES")
                        .desc("list of packages to install in shell");
                passthrough_args =
                    extra("ARGS")
                        .desc("Additional arguments to pass to `nix shell`");
            } in {
                Self {
                    dryrun,
                    override_config_file,
                    override_flake_lockfile,
                    packages,
                    passthrough_args,
                }
            }
        }
    }
}

pub enum Override {
    ConfigFile(PathBuf),
    FlakeLockfile(String),
}

pub struct Args {
    pub dryrun: bool,
    pub override_: Option<Override>,
    pub packages: Vec<String>,
    pub passthrough_args: Vec<String>,
}

impl Args {
    pub fn parse() -> Self {
        use meap::Parser;
        let CliArgs {
            dryrun,
            override_config_file,
            override_flake_lockfile,
            packages,
            passthrough_args,
        } = CliArgs::parser()
            .with_help_default()
            .with_program_description(
                "Start a transient shell with some specified packages installed.\n\
                Packages are installed from the nixpkgs repo matching the revision from a flake.lock file.\n\
                Intended to be used to temporarily test out packages without committing to installing them,\n\
                and to guarantee that the packages are compatible with system-wide or home-manager configs\n\
                managed with flakes.\n\
                \n\
                Configure with a file ~/.config/nix-shell-locked.toml, e.g.:\n\
                flake_lockfile = \"/path/to/flake.lock\"\n\
                \n\
                Read more at https://github.com/gridbugs/nix-shell-locked
                "
            )
            .with_version_default(env!("CARGO_PKG_VERSION"))
            .parse_env_or_exit();
        if override_config_file.is_some() && override_flake_lockfile.is_some() {
            eprintln!("Specify at most one of --config and --lockfile");
            process::exit(1);
        }
        let override_ = override_config_file
            .map(Override::ConfigFile)
            .or(override_flake_lockfile.map(Override::FlakeLockfile));
        Self {
            dryrun,
            override_,
            packages,
            passthrough_args,
        }
    }
}
