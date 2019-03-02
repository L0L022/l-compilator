use std::fmt;

#[derive(Debug)]
struct SymbolTable<'a> {
    tab: Vec<Symbol<'a>>,
    base: i32,
    sommet: i32,
}

#[derive(Debug)]
struct Symbol<'a> {
    id: &'a str,
    address: i32,
    kind: SymbolKind,
}

#[derive(Debug, Copy, Clone)]
enum Scope {
    Global,
    Local,
    Argument,
}

#[derive(Debug, Copy, Clone)]
enum SymbolKind {
    Scalar { scope: Scope },
    Vector { scope: Scope, size: i32 },
    Function { nb_arguments: i32 },
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Scope::*;

        match self {
            Global => write!(f, "GLOBALE"),
            Local => write!(f, "LOCALE"),
            Argument => write!(f, "ARGUMENT"),
        }
    }
}

impl<'a> fmt::Display for SymbolTable<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "------------------------------------------")?;
        writeln!(f, "base = {}", self.base)?;
        writeln!(f, "sommet = {}", self.sommet)?;
        for (i, e) in self.tab.iter().enumerate() {
            write!(f, "{} {}", i, e)?;
        }
        writeln!(f, "------------------------------------------")
    }
}

impl<'a> fmt::Display for Symbol<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use SymbolKind::*;

        let (scope, kind, additional) = match self.kind {
            Scalar { scope } => (scope, "ENTIER", 1),
            Vector { scope, size } => (scope, "TABLEAU", size),
            Function { nb_arguments } => (Scope::Global, "FONCTION", nb_arguments),
        };

        writeln!(
            f,
            "{} {} {} {} {}",
            self.id, scope, kind, self.address, additional
        )
    }
}
