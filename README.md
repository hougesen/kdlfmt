# kdlfmt

A formatter for [kdl](https://kdl.dev) documents.

kdlfmt is a thin cli wrapper built on-top of the official `kdl` parser for Rust ([`kdl-rs`](https://github.com/kdl-org/kdl-rs)), so any formatting/parsing issues should be reported there.

<!-- START_SECTION:base-command-help -->

```
kdlfmt 0.1.0
A code formatter for kdl documents.
Mads Hougesen <mads@mhouge.dk>

Usage: kdlfmt [OPTIONS] <COMMAND>

Commands:
  format       Format kdl files
  check        Validate files are formatted
  init         Initialize formatter config
  completions  Generate shell completions
  help         Print this message or the help of the given subcommand(s)

Options:
      --log-level <LOG_LEVEL>  [possible values: trace, debug, info, warn, error, off]
  -h, --help                   Print help
  -V, --version                Print version
```

<!-- END_SECTION:base-command-help -->

## Installation

### Using Cargo

kdlfmt can be installed using Cargo, the package manager for Rust ([crates.io](https://crates.io/crates/kdlfmt)).

```shell
cargo install kdlfmt
```

### Using Homebrew

If you're on macOS or Linux, you can install kdlfmt using Homebrew:

```shell
# Tap and install
brew tap hougesen/tap
brew install kdlfmt

# Or install directly in one command
brew install hougesen/tap/kdlfmt
```

### Using npm/npx

You can install `kdlfmt` using [npm](https://www.npmjs.com/package/kdlfmt):

```shell
npm install -g kdlfmt

kdlfmt format .
```

Or run it directly using npx:

```shell
npx kdlfmt format .
```

### Precompiled Binaries

If you do not have/want Rust or Homebrew installed on your device, you can find precompiled binaries on the [release](https://github.com/hougesen/kdlfmt/releases) page or run one of the installers below.

#### Linux & MacOS

```shell
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/hougesen/kdlfmt/releases/latest/download/kdlfmt-installer.sh | sh
```

#### Windows

```powershell
powershell -ExecutionPolicy ByPass -c "irm https://github.com/hougesen/kdlfmt/releases/latest/download/kdlfmt-installer.ps1 | iex"
```

## Other package managers

A non-complete list of other package managers with support for installing kdlfmt can be found at [Repology](https://repology.org/project/kdlfmt).

[![Packaging status](https://repology.org/badge/vertical-allrepos/kdlfmt.svg?columns=3)](https://repology.org/project/kdlfmt/versions)

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

      --config <CONFIG>
          Path to config file

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

      --config <CONFIG>
          Path to config file

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

### GitHub Action

There are a lot of different ways to run `kdlfmt` using GitHub actions.

The easiest way, in my opinion, is to use the official GitHub action to install `kdlfmt`.

After that you can run the binary like you would in your terminal.

```yaml
name: kdlfmt
on:
  - push
jobs:
  format:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Install kdlfmt
        uses: hougesen/kdlfmt@main

      - name: Run kdlfmt format
        run: kdlfmt format .

      - name: Commit changes
        uses: EndBug/add-and-commit@v9
        with:
          message: "style: formatted kdl"
```

### Shell completion

Shell completion can be generated using the `kdlfmt completions` command.

<!-- START_SECTION:completions-command-help -->

```
Generate shell completions

Usage: kdlfmt completions [OPTIONS] <SHELL>

Arguments:
  <SHELL>  [possible values: bash, elvish, fish, nushell, powershell, zsh]

Options:
      --log-level <LOG_LEVEL>  [possible values: trace, debug, info, warn, error, off]
  -h, --help                   Print help
  -V, --version                Print version
```

<!-- END_SECTION:completions-command-help -->

#### Bash

Add the following to your `.bashrc`.

```bash
eval "$(kdlfmt completions bash)"
```

#### Zsh

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

#### Nushell

Generate completions for [nushell](https://github.com/nushell/nushell).

```nushell
kdlfmt completions nushell
```
