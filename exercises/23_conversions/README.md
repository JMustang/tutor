# Conversões de tipo

Rust oferece diversas maneiras de converter um valor de um determinado tipo em outro.

A forma mais simples de conversão de tipo é uma expressão de conversão de tipo. Ela é denotada pelo operador binário `as`. Por exemplo, `println!("{}", 1 + 1.0);` não seria compilado, pois `1` é um inteiro enquanto `1.0` é um float. No entanto, `println!("{}", 1 as f32 + 1.0)` deve ser compilado. O exercício [`using_as`](using_as.rs) tenta abordar esse assunto.

Rust também oferece traits que facilitam as conversões de tipo na implementação. Essas traits podem ser encontradas no módulo [`convert`](https://doc.rust-lang.org/std/convert/index.html).

As traits são as seguintes:

- `From` e `Into`, abordadas em [`from_into`](from_into.rs)
- `TryFrom` e `TryInto`, abordadas em [`try_from_into`](try_from_into.rs)
- `AsRef` e `AsMut`, abordadas em [`as_ref_mut`](as_ref_mut.rs)

Além disso, o módulo `std::str` oferece uma trait chamada [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) que auxilia na conversão de strings para os tipos desejados através do método `parse` em strings. Se implementado corretamente para um dado tipo `Person`, então `let p: Person = "Mark,20".parse().unwrap()` deve compilar e executar sem erros.

Essas devem ser as principais maneiras ***dentro da biblioteca padrão*** de converter dados para os tipos desejados. ## Informações adicionais

Esses tópicos não são abordados diretamente no livro, mas a biblioteca padrão possui uma excelente documentação sobre eles.

- [conversions](https://doc.rust-lang.org/std/convert/index.html)
- [`FromStr` trait](https://doc.rust-lang.org/std/str/trait.FromStr.html)
