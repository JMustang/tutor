// Os testes são importantes para garantir que seu código faça o que você espera que ele faça.

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: Importe `is_even`. Você pode usar um caractere curinga para importar tudo em
    //TODO: um módulo externo.

    use crate::is_even;

    #[test]
    fn you_can_assert() {
        // TODO: Teste a função `is_even` com alguns valores.
        assert!(is_even(2));
        assert!(!is_even(3));
    }
}
