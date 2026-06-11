// Em tempo de compilação, o Rust precisa saber quanto espaço um tipo ocupa. Isso
// se torna problemático para tipos recursivos, onde um valor pode ter como parte de
// si mesmo outro valor do mesmo tipo. Para contornar o problema, podemos usar um
// `Box` - um ponteiro inteligente usado para armazenar dados no heap, que também nos permite
// encapsular um tipo recursivo.
// O tipo recursivo que estamos implementando neste exercício é a "lista cons", uma
// estrutura de dados frequentemente encontrada em linguagens de programação funcional. Cada
// item em uma lista cons contém dois elementos: o valor do item atual e
// o próximo item. O último item é um valor chamado `Nil`.

// TODO: Use um `Box` na definição do enum para que o código seja compilado.
#[derive(PartialEq, Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// TODO: Crie uma lista de contras vazia.
fn create_empty_list() -> List {
    List::Nil
}

// TODO: Crie uma lista de contras não vazia.
fn create_non_empty_list() -> List {
    List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))))
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(create_empty_list(), List::Nil);
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list());
    }
}
