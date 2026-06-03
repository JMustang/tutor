fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Transforme isso em uma instrução if-let cujo valor seja `Some`.
        if let Some(word) = optional_target {
            println!("The word is: {word}");

            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: Transforme isso em uma instrução while-let. Lembre-se de que `Vec::pop()`
        // TODO: adiciona outra camada de `Option`. Você pode fazer correspondência de padrões aninhada
        // TODO: em instruções if-let e while-let.
        while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
