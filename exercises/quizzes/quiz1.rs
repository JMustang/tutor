// Este é um questionário para as seguintes seções:
// - Variáveis
// - Funções
// - Se
//
// Maria está comprando maçãs. O preço de uma maçã é calculado da seguinte forma:
// - Uma maçã custa 2 rustbucks.
// - No entanto, se Maria comprar mais de 40 maçãs, o preço de cada maçã em
// todo o pedido é reduzido para apenas 1 rustbuck!

// TODO: Escreva uma função que calcule o preço de um pedido de maçãs dada
// a quantidade comprada.

// fn calcular_preço_de_maçãs(???) -> ??? { ??? }
fn calcular_preço_de_maçãs(quantidade: u32) -> u32 {
    if quantidade > 40 {
        quantidade
    } else {
        quantidade * 2
    }
}

fn main() {
    // You can optionally experiment here.
}

// Não mude esse teste! Ele é usado para verificar se sua função está correta. Se o teste falhar, ele mostrará uma mensagem de erro indicando o que deu errado.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calcular_preço_de_maçãs(35), 70);
        assert_eq!(calcular_preço_de_maçãs(40), 80);
        assert_eq!(calcular_preço_de_maçãs(41), 41);
        assert_eq!(calcular_preço_de_maçãs(65), 65);
    }
}
