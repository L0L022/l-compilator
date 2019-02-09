use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(author = "Loïc Escales <loic.escales@etu.univ-amu.fr>")]
pub struct Opt {
    /// Affiche les tokens de l'analyse lexicale
    #[structopt(short = "l")]
    pub lex: bool,

    /// Affiche l'arbre abstrait
    #[structopt(short = "a")]
    pub ast: bool,

    /// Affiche la table des symboles
    #[structopt(short = "t")]
    pub symbol_table: bool,

    /// Affiche le code trois adresses
    #[structopt(short = "3")]
    pub three_address_code: bool,

    /// Affiche le code nasm (actif par defaut)
    #[structopt(short = "n")]
    pub nasm: bool,

    /// Le fichier l source
    #[structopt(parse(from_os_str))]
    pub source_file: PathBuf,
}
