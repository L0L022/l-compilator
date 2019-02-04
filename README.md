# l-compilator

Un compilateur pour le langage l écrit en Rust.

## Compilation

D'abord il faut installer [cargo](https://rustup.rs/), puis :

```bash
# Build
cd l-compilator
cargo build --release

# Optional: run unit tests and integration tests
cargo test

# Run without install
cargo run --release -- -h

# Install
cargo install --path .
```

## Les options

```
USAGE:
    l-compilator [FLAGS] <source_file>

FLAGS:
    -a               Affiche l'arbre abstrait
    -h, --help       Prints help information
    -l               Affiche les tokens de l'analyse lexicale
    -n               Affiche le code nasm (actif par defaut)
    -t               Affiche la table des symboles
    -3               Affiche le code trois adresses
    -V, --version    Prints version information

ARGS:
    <source_file>    Le fichier l source
```
