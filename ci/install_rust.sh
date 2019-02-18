set -ex

install_rust() {
    chmod u+x ci/install_curl.sh && source ./ci/install_curl.sh
    curl https://sh.rustup.rs -sSf | sh -s -- -y
    rustup target add $TARGET
    rustc --version && cargo --version      # Print version info for debugging
}

main() {
    mkdir -p .rustup_cache
    mkdir -p .cargo_cache

    # Only stuff inside the repo directory can be cached
    # Override the CARGO_HOME variable to force it location
    export RUSTUP_HOME="${PWD}/.rustup_cache"
    export CARGO_HOME="${PWD}/.cargo_cache"
    export PATH="$CARGO_HOME/bin:$PATH"

    (rustc --version && cargo --version) || install_rust
}

main
