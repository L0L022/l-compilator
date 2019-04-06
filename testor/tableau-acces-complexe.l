entier tableau[3], entier a, entier b, entier c;

f() {
  retour 0;
}

main() {
  tableau[f()] = lire();
  tableau[lire()] = 1;
  tableau[tableau[0]] = 0;
  a = tableau[f()];
  b = tableau[3+2-4];
  c = tableau[lire()];
  ecrire(tableau[0]);
  ecrire(tableau[1]);
  ecrire(tableau[2]);
  ecrire(a);
  ecrire(b);
  ecrire(c);
}
