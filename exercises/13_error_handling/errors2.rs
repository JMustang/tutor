// Digamos que estamos escrevendo um jogo onde você pode comprar itens com fichas. Todos os itens custam
// 5 fichas, e sempre que você compra itens, há uma taxa de processamento de 1
// ficha. Um jogador digitará quantos itens deseja comprar, e
// a função `total_cost` calculará o custo total dos itens. Como
// o jogador digitou a quantidade, a recebemos como uma string. Ele pode ter
// digitado qualquer coisa, não apenas números!
// No momento, esta função não está tratando o caso de erro. O que queremos
// fazer é: Se chamarmos a função `total_cost` em uma string que não seja um
// número, essa função retornará um `ParseIntError`. Nesse caso, queremos
// retornar imediatamente esse erro da nossa função e não tentar multiplicar e
// somar.
// Existem pelo menos duas maneiras de implementar isso que estão corretas. Mas uma
// é muito mais curta!

use std::num::ParseIntError;

fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;

    // TODO: Trate o caso de erro conforme descrito acima.
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::IntErrorKind;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().kind(),
            &IntErrorKind::InvalidDigit,
        );
    }
}
