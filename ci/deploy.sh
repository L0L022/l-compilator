set -ex

install_python_ppa() {
    apt-get update -qq && apt-get install -y -qq python-software-properties
    add-apt-repository -y ppa:fkrull/deadsnakes
    apt-get update -qq && apt-get install -y -qq python3.5
}

install_python() {
    apt-get update -qq
    apt-get install -y -qq python3.5 || install_python_ppa
    python3.5 --version
}

install_pip3() {
    install_python
    chmod u+x ci/install_curl.sh && source ./ci/install_curl.sh
    curl -sS https://bootstrap.pypa.io/get-pip.py | python3.5
    pip3 --version
}

main() {
    local src=$(pwd) \
          stage=$(mktemp -d)

    test -f Cargo.lock || cargo generate-lockfile

    # TODO Update this to build the artifacts that matter to you
    cargo rustc --bin $BIN_NAME --target $TARGET --release -- -C lto

    # TODO Update this to package the right artifacts
    cp target/$TARGET/release/$BIN_NAME $stage/ || cp target/$TARGET/release/$BIN_NAME.exe $stage/

    local package="$src/$CRATE_NAME-$CI_COMMIT_TAG-$TARGET"
    cd $stage

    if echo $TARGET | grep -q "windows"
    then
        apt-get update -qq && apt-get install -y -qq zip
        package="$package.zip"
        zip -r $package *
    else
        package="$package.tar.gz"
        tar czf $package *
    fi

    cd $src
    rm -rf $stage

    install_pip3
    pip3 install gitlab_release
    python3.5 -m gitlab_release $PRIVATE_TOKEN $package
}

main
