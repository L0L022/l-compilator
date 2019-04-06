true() {
  retour 1 = 1;
}

false() {
  retour 0 = 1;
}

equalator(entier a, entier b)
{
  si a = b alors
  {
    ecrire(0);
  }
  sinon
  {
    ecrire(1);
  }
}

main()
{
  equalator(true(), true());
  equalator(false(), false());
  equalator(false(), true());
  equalator(true(), false());
  equalator(false(), !true());
  equalator(!true(), false());
  equalator(true(), !false());
  equalator(!false(), true());
  equalator(true() & true(), true());
  equalator(true() & false(), false());
  equalator(false() & true(), false());
  equalator(false() & false(), false());
  equalator(true() | true(), true());
  equalator(true() | false(), true());
  equalator(false() | true(), true());
  equalator(false() | false(), false());
}
