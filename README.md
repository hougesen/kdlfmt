# kdlfmt

A formatter for [kdl](https://kdl.dev) documents.

kdlfmt is a thin cli wrapper built on-top of the official `kdl` parser for Rust ([`kdl-rs`](https://github.com/kdl-org/kdl-rs)), so any formatting/parsing issues should be reported there.

<!-- START_SECTION:base-command-help -->

```
kdlfmt 0.0.15
A code formatter for kdl documents.
Mads Hougesen <mads@mhouge.dk>

Usage: kdlfmt <COMMAND>

Commands:
  format       Format kdl files
  check        Validate files are formatted
  init         Initialize formatter config
  completions  Generate shell completions
  help         Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```

<!-- END_SECTION:base-command-help -->

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

Or reading from stdin and printing the formatted output to stdout.

```shell
cat somefile.kdl | kdlfmt format -
```

<!-- START_SECTION:format-command-help -->

```
Format kdl files

Usage: kdlfmt format [OPTIONS] [INPUT]...

Arguments:
  [INPUT]...
          Path to file OR directory.

          Use "-" to read from stdin and print to stdout.

Options:
      --kdl-version <KDL_VERSION>
          kdl specification to use.

          By default all versions are tried

          [possible values: v1, v2]

      --stdin
          Read from stdin and print to stdout

      --log-level <LOG_LEVEL>
          [possible values: trace, debug, info, warn, error, off]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```

<!-- END_SECTION:format-command-help -->

### Validating files are formatted

kdlfmt also support validating if files are formatted using the `kdlfmt check` command.

```shell
kdlfmt check PATH
```

Or reading from stdin.

```shell
cat somefile.kdl | kdlfmt check -
```

<!-- START_SECTION:check-command-help -->

```
Validate files are formatted

Usage: kdlfmt check [OPTIONS] [INPUT]...

Arguments:
  [INPUT]...
          Path to file OR directory.

          Use "-" to read from stdin and print to stdout.

Options:
      --kdl-version <KDL_VERSION>
          kdl specification to use.

          By default all versions are tried

          [possible values: v1, v2]

      --stdin
          Read from stdin and print to stdout

      --log-level <LOG_LEVEL>
          [possible values: trace, debug, info, warn, error, off]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```

<!-- END_SECTION:check-command-help -->

### Ignoring files

`.kdlfmtignore` files are used to ignore files/directories.

It uses the same syntax as `.gitignore` files.

### Shell completion

Shell completion can be generated using the `kdl completions` command.

<!-- START_SECTION:completions-command-help -->

```
Generate shell completions

Usage: kdlfmt completions <SHELL>

Arguments:
  <SHELL>  [possible values: bash, elvish, fish, powershell, zsh]

Options:
  -h, --help     Print help
  -V, --version  Print version

```

<!-- END_SECTION:completions-command-help -->

#### Bash

Add the following to your `.bashrc`.

```bash
eval "$(kdlfmt completions bash)"
```

#### Bash

Add the following to your `.zshrc`:

```zsh
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
