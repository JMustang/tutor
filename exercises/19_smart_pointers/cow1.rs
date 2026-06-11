// Este exercício explora o ponteiro inteligente `Cow` (Clone-On-Write). Ele pode
// encapsular e fornecer acesso imutável a dados emprestados e clonar os dados
// de forma preguiçosa quando mutação ou propriedade forem necessárias. O tipo foi projetado para funcionar
// com dados emprestados em geral por meio da trait `Borrow`.

use std::borrow::Cow;

fn abs_all(input: &mut Cow<[i32]>) {
    for ind in 0..input.len() {
        let value = input[ind];
        if value < 0 {
            // Clona em um vetor caso ainda não o possua.
            input.to_mut()[ind] = -value;
        }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow::Owned;

    use super::*;

    #[test]
    fn reference_mutation() {
        // Clone occurs because `input` needs to be mutated.
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        assert!(matches!(input, Cow::Owned(_)));
    }

    #[test]
    fn reference_no_mutation() {
        // Não ocorre nenhum clone porque `input` não precisa ser modificado.
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        // TODO: Substitua `todo!()` por `Cow::Owned(_)` ou `Cow::Borrowed(_)`.
        assert!(matches!(input, Cow::Borrowed(_)));
    }

    #[test]
    fn owned_no_mutation() {
        // Também podemos passar `vec` sem `&` para que `Cow` o possua diretamente. Nesse caso,
        // também não há clonagem. Mas o resultado ainda é de propriedade de alguém, porque nunca foi
        // nenhuma mutação ocorre (todos os números já são absolutos) e, portanto,
        // emprestado ou mutado.
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        // TODO: Substitua `todo!()` por `Cow::Owned(_)` ou `Cow::Borrowed(_)`.
        assert!(matches!(input, Cow::Owned(_)));
    }

    #[test]
    fn owned_mutation() {
        // É claro que isso também se aplica se ocorrer uma mutação (nem todos os
        // função `abs_all` retorna uma referência aos mesmos dados de antes.
        // números são absolutos). Nesse caso, a chamada para `to_mut()` na
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        // TODO: Substitua `todo!()` por `Cow::Owned(_)` ou `Cow::Borrowed(_)`.
        assert!(matches!(input, Owned(_)));
    }
}
