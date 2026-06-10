fn factorial(num: u64) -> u64 {
    // TODO: Complete esta função para retornar o fatorial de `num`, que é
    // TODO: definido como `1 * 2 * 3 * … * num`.
    // TODO: https://en.wikipedia.org/wiki/Factorial
    // TODO: Não use:
    // TODO: - retornos antecipados (usando a palavra-chave `return` explicitamente)
    // TODO: Tente não usar:
    // TODO: - loops imperativos (for/while)
    // TODO: - variáveis ​​adicionais
    // TODO: Para um desafio extra, não use:
    // TODO: - recursão
    (1..=num).product()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial(1), 1);
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial(4), 24);
    }
}
