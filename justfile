alias b := build
alias l := lint
alias t := test
alias tc := test-coverage

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
    cargo clippy --all-targets --all-features -- -Dclippy::style -Dclippy::double_neg -Dclippy::perf -Dclippy::pedantic -Dclippy::all -Dclippy::cargo -Dclippy::complexity -Dclippy::nursery -Dclippy::suspicious -Aclippy::module_name_repetitions -Aclippy::missing_errors_doc -Aclippy::must_use_candidate -Aclippy::multiple_crate_versions
    cargo clean

test:
    just lint
    RUST_BACKTRACE=full cargo test --release

test-coverage:
    cargo llvm-cov clean
    cargo llvm-cov --all-features --open

format:
    cargo fmt
    just --fmt --unstable .
    mdsf format .
    npx prettier --write --cache .

changelog:
    npx auto-changelog

precommit:
    cargo clean
    just changelog
    just format
    just build
    just lint
    just test
    typos --exclude CHANGELOG.md .

publish:
    just build
    just lint

    cargo clean
    cargo publish
