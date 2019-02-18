set -ex

install_curl() {
    apt-get update -qq && apt-get install -y -qq curl
    curl --version
}

main() {
    curl --version || install_curl
}

main
