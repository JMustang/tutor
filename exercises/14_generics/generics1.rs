// `Vec<T>` é genérico sobre o tipo `T`. Na maioria dos casos, o compilador consegue
// inferir `T`, por exemplo, após inserir um valor com um tipo concreto no vetor.
// Mas neste exercício, o compilador precisa de alguma ajuda por meio de uma anotação de tipo.

fn main() {
    // TODO: Corrigir o erro de compilação anotando o tipo do vetor
    // TODO:  `Vec<T>`. Escolha `T` como algum tipo inteiro que possa ser criado a partir de
    //TODO: `u8` e `i8`.

    let mut numbers = Vec::<i8>::new();

    // Don't change the lines below.
    let n1: i8 = 42;
    numbers.push(n1.into());
    let n2: i8 = -1;
    numbers.push(n2.into());

    println!("{numbers:?}");
}
