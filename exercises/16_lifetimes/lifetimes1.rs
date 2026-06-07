// O compilador Rust precisa saber como verificar se as referências fornecidas são
// válidas, para que possa informar ao programador se uma referência corre o risco de
// sair do escopo antes de ser usada. Lembre-se, referências são empréstimos e
// não possuem seus próprios dados. E se o proprietário delas sair do escopo?

// TODO: Corrigir o erro do compilador atualizando a assinatura da função.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert_eq!(longest("abcd", "123"), "abcd");
        assert_eq!(longest("abc", "1234"), "1234");
    }
}
