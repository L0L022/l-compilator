ecrire0()
{
  ecrire(0);
  retour 0 = 1;
}

ecrire1()
{
  ecrire(1);
  retour 0 = 1;
}

main()
entier r;
{
  r = ecrire0() & ecrire1();
  ecrire(2);
}
