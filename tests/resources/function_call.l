# https://gitlab.com/louwaque/l-compilator/issues/4
main()
{
    ecrire(a);
    a = lire();

    func();
    func(a);
    func(a, b, c);

    a = func();
    a = func(a);
    a = func(a, b, c);
}
