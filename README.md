# kdlfmt

A formatter for [kdl](https://kdl.dev) documents.

kdlfmt is a thin cli wrapper built on-top of the official `kdl` parser for Rust ([`kdl-rs`](https://github.com/kdl-org/kdl-rs)), so any formatting/parsing issues should be reported there.

## Installation

kdlfmt can be installed using Cargo, the package manager for Rust ([crates.io](https://crates.io/crates/kdlfmt)).

```shell
cargo install kdlfmt
```

If you do not have/want Rust installed on your device you can find precompiled binaries on the [release](https://github.com/hougesen/kdlfmt/releases) page.

## Usage

Once installed the formatted can be invoked by running `kdlfmt format`.

```shell
kdlfmt format PATH
```

kdlfmt also support validating if files are formatted using the `kdlfmt check` command.

```shell
kdlfmt check PATH
```

### Ignoring files

`.kdlfmtignore` files are used to ignore files/directories.

It uses the same syntax as `.gitignore` files.

### Shell completion

Shell completion can be generated using the `kdl completions` command.

#### Bash

Add the following to your `.bashrc`.

```bash
eval "$(kdlfmt completions bash)"
```

#### Bash

Add the following to your `.zshrc`:

```bash
eval "$(kdlfmt completions zsh)"
```

#### Fish

Add the following to `~/.config/fish/config.fish`.

```fish
kdlfmt completions fish | source
```

#### PowerShell

Add the following to your PowerShell configuration (Can be found by running `$PROFILE`).

```powershell
Invoke-Expression (&kdlfmt completions powershell)
```

#### Elvish

Add the following to `~/.elvish/rc.elv`.

```elvish
eval (kdlfmt completions elvish)
```
