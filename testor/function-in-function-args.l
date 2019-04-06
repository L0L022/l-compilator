fn1(entier a) {
  retour a;
}

fn2(entier a, entier b) {
  retour a + b;
}

fn3(entier a, entier b, entier c) {
  retour a + b + c;
}

main() {
  ecrire(fn3(fn1(1), fn2(fn1(1), fn2(1, 1)), fn2(fn3(1, 1, 1), fn2(fn1(1), 1))));
}
