// É necessário definir uma cesta de frutas na forma de um mapa hash. A chave
// representa o nome da fruta e o valor representa quantas
// frutas específicas há na cesta. Você deve colocar pelo menos 3 tipos diferentes de frutas
// (por exemplo, maçã, banana, manga) na cesta e a contagem total
// de todas as frutas deve ser de pelo menos 5.

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    // TODO: Declare o mapa hash.
    // let mut basket =
    let mut basket = HashMap::<String, u32>::new();

    // TODO:Já te demos duas bananas :)
    //La ele.
    basket.insert(String::from("banana"), 2);

    // TODO: Coloque mais frutas na sua cesta.
    basket.insert(String::from("Maçã"), 3);
    basket.insert(String::from("pera"), 1);
    basket.insert(String::from("Manga"), 1);

    basket
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
