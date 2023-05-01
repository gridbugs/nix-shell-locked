# Nix Shell Locked

[![Latest Version](https://img.shields.io/crates/v/nix-shell-locked.svg)](https://crates.io/crates/nix-shell-locked)

`nix-shell-locked` is a program which starts a new shell with some specified
packages available without installing them user or system wide. Packages are
installed from a revision of [nixpkgs](https://github.com/NixOS/nixpkgs/) taken
from a `flake.lock` file associated with a system or home-manager config. Reads
a config file at `~/.config/nix-shell-locked.toml` to find out which flake
lockfile to look in to determine the nixpkgs revision to use.

```toml
# ~/.config/nix-shell-locked.toml
flake_lockfile = "/path/to/config/repo/flake.lock"
```

This is intended as a replacement for `nix-shell` in cases where flakes is used
to manage either a NixOS system configuration or home-manager. The problem with
`nix-shell` is that it uses the nixpkgs channel which can get out sync with the
version of nixpkgs in a flake-managed system or home-manager config which can
lead to runtime errors.

## Quick Example

```bash
# initially `cowsay` is not installed
$ cowsay
bash: cowsay: command not found

# enter a new shell with cowsay available
$ nix-shell-locked cowsay

# now `cowsay` is installed (but only in this shell session)
$ cowsay "Hello, World!"
 _______________
< Hello, World! >
 ---------------
        \   ^__^
         \  (oo)\_______
            (__)\       )\/\
                ||----w |
                ||     ||

# leave the shell session
$ exit
exit

# now `cowsay` is once again unavailable
$ cowsay
bash: cowsay: command not found
```

## Installation

### Install flake in nix profile
```
$ nix profile install github:gridbugs/nix-shell-locked
```

### Install with cargo
```
$ cargo install nix-shell-locked
```

### Install with flakes-based home-manager

See an example configuration [here](https://github.com/gridbugs/dotfiles/tree/94317accaaea260107cf9eba19202e3c802e77b0/home-manager/ebrietas).

## Usage

```
$ nix-shell-locked --help
Usage: nix-shell-locked [OPTIONS] [PACKAGES ...] -- [ARGS ...]
Start a transient shell with some specified packages installed.
Packages are installed from the nixpkgs repo matching the revision from a flake.lock file.
Intended to be used to temporarily test out packages without committing to installing them,
and to guarantee that the packages are compatible with system-wide or home-manager configs
managed with flakes.

Configure with a file ~/.config/nix-shell-locked.toml, e.g.:
flake_lockfile = "/path/to/flake.lock"

Read more at https://github.com/gridbugs/nix-shell-locked


Args:
    [PACKAGES ...]     list of packages to install in shell

Options:
    [--dry-run]               print the command that would be executed instead of executing it
    [-c, --config PATH]       path to config file to use (defaults to $XDG_CONFIG_HOME/nix-shell-locked.toml)
    [-l, --lockfile PATH]     path to flake lockfile to use when determining nixpkgs revision
    [-h, --help]              print help message
    [-v, --version]           print version information

Extra Args:
    [ARGS ...]    Additional arguments to pass to `nix shell`
```

## Detailed Example

I use flakes to manage both my NixOS system configuration and home-manager. I
want to try out the gameboy emulator "sameboy" but I don't want to commit to
installing it system-wide or adding it to my home-manager config, so I install
it in a transient shell with:

```
$ nix-shell -p sameboy
```

When I try to run it:

```
$ sameboy
SameBoy v0.15.8
Couldn't find matching GLX visual
```

This looks like a version mismatch between a graphics library and sameboy.
Graphics libraries are configured in the system-wide configuration which I
manage with flakes which I update regularly. Since I use flakes for almost
everything now I neglect to keep my channels up to date, so this version of
sameboy is likely quite old. Ideally there would be a way to install sameboy in
a transient shell where the version of sameboy comes from the same nixpkgs
revision as the system-wide configuration, and there is.

My system config has a flake.lock file with a section:

```js
// /path/to/config/repo/flake.lock
{
  "nodes": {
    "nixpkgs": {
      "locked": {
        "lastModified": 1682268651,
        "narHash": "sha256-2eZriMhnD24Pmb8ideZWZDiXaAVe6LzJrHQiNPck+Lk=",
        "owner": "nixos",
        "repo": "nixpkgs",
        "rev": "e78d25df6f1036b3fa76750ed4603dd9d5fe90fc",
        "type": "github"
      },

```

Note the revision `e78d25df6f1036b3fa76750ed4603dd9d5fe90fc`.

We can make a transient shell with sameboy installed from this revision with:

```
$ nix shell nixpkgs/e78d25df6f1036b3fa76750ed4603dd9d5fe90fc#sameboy --command sameboy
```

This downloads and runs sameboy, taking a version that is compatible with the
system-wide graphics library installation. This works but it's cumbersome. The
`nix-shell-locked` command automates the above process. Make a config file `~/.config/nix-shell-locked.toml` to
tell `nix-shell-locked` where to look for the `flake.lock` file to use to
obtain the current revision hash.

```toml
# ~/.config/nix-shell-locked.toml
flake_lockfile = "/path/to/config/repo/flake.lock"
```

Now you can start a transient shell with the correct version of sameboy by running:
```
$ nix-shell-locked sameboy
```

Behind the scenes this is just running `nix shell ...` and all arguments after
the first `--` are passed along to `nix shell`, so you can do:
```
$ nix-shell-locked sameboy -- --command sameboy --help
SameBoy v0.15.8
Usage: sameboy [--fullscreen|-f] [--nogl] [--stop-debugger|-s] [rom]
```

You can pass multiple packages to `nix-shell-locked` to get a shell with all packages available:
```
$ nix-shell-locked ksh hello cowsay -- --command ksh -c "hello | cowsay -f tux"
 _______________
< Hello, world! >
 ---------------
   \
    \
        .--.
       |o_o |
       |:_/ |
      //   \ \
     (|     | )
    /'\_   _/`\
    \___)=(___/
```
