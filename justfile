build:
    cargo check
    cargo build
    cargo build --release

build-local:
    just build
    sudo cp target/release/kdlfmt /usr/local/bin/kdlfmt-local

lint:
    cargo fmt -- --check --color always
    cargo clippy --all-targets --all-features

lint-aggressive:
    cargo clean
    cargo clippy --fix --allow-staged --all-targets --all-features -- -Dclippy::style -Dclippy::perf -Dclippy::pedantic -Dclippy::all -Dclippy::cargo -Dclippy::complexity -Dclippy::nursery -Dclippy::suspicious -Aclippy::module_name_repetitions -Aclippy::missing_errors_doc -Aclippy::must_use_candidate -Aclippy::multiple_crate_versions -Aclippy::needless_raw_strings -Aclippy::needless_raw_string_hashes
    cargo clean

test:
    just lint
    RUST_BACKTRACE=full cargo test --release

test-coverage:
    cargo llvm-cov clean
    cargo llvm-cov --all-features --open

format:
    ruff format
    cargo +nightly fmt
    just --fmt --unstable .
    mdsf format .
    npx prettier --write --cache .

changelog:
    npx auto-changelog -u --hide-credit -l 100 -b 100

update-readme-command-help:
    python3 readme-command-help.py

precommit-github-action:
    cd github-action && npm i
    cd github-action && npm run lint:biome:fix
    cd github-action && npm run lint:eslint:fix
    cd github-action && npm run build

precommit:
    just precommit-github-action 
    just changelog
    just update-readme-command-help
    just format
    just build
    just lint
    just test
    typos --exclude CHANGELOG.md --exclude github-action/dist .
    dist init --yes
