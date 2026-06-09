struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    // Don't change this function.
    fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            // Retornar um `Result` seria melhor aqui. Mas queremos aprender
            // como testar funções que podem entrar em pânico.
            panic!("A largura e a altura do retângulo devem ser positivas.");
        }

        Rectangle { width, height }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // TODO: Este teste deve verificar se o retângulo tem o tamanho que nós
        // TODO: passamos para o seu construtor.
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10); // Check width
        assert_eq!(rect.height, 20); // Check height
    }

    // TODO: Este teste deve verificar se o programa entra em pânico quando tentamos criar
    // TODO: um retângulo com largura negativa.
    #[test]
    #[should_panic(expected = "A largura e a altura do retângulo devem ser positivas.")]
    fn negative_width() {
        let _rect = Rectangle::new(-10, 10);
    }

    // TODO: Este teste deve verificar se o programa entra em pânico quando tentamos criar
    // TODO: um retângulo com altura negativa.
    #[test]
    #[should_panic(expected = "A largura e a altura do retângulo devem ser positivas.")]
    fn negative_height() {
        let _rect = Rectangle::new(10, -10);
    }
}
