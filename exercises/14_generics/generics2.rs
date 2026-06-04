// Este poderoso wrapper oferece a capacidade de armazenar um valor inteiro positivo.
// TODO: Reescrevê-lo usando um genérico para que suporte o encapsulamento de QUALQUER tipo.
struct Wrapper<T> {
    value: T,
}

// TODO: Adapte a implementação da estrutura para que seja genérica em relação ao valor encapsulado.
impl<T> Wrapper<T> {
    fn new(value: T) -> Self {
        Wrapper { value }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42u32).value, 42u32);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
