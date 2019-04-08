set -ex

install_clang_precise() {
    chmod u+x ci/install_curl.sh && source ./ci/install_curl.sh
    echo "deb http://apt.llvm.org/precise/ llvm-toolchain-precise-3.9 main" >> /etc/apt/sources.list
    echo "deb-src http://apt.llvm.org/precise/ llvm-toolchain-precise-3.9 main" >> /etc/apt/sources.list
    curl https://apt.llvm.org/llvm-snapshot.gpg.key -sSf | apt-key add -
    apt-get install -y -qq python-software-properties && add-apt-repository -y ppa:ubuntu-toolchain-r/test
    apt-get update -qq && apt-get install -y -qq llvm-3.9-dev libclang-3.9-dev clang-3.9
}

install_clang() {
  apt-get install -y -qq llvm-3.9-dev libclang-3.9-dev clang-3.9 || install_clang_precise
}

main() {
    clang-3.9 --version || install_clang
}

main
