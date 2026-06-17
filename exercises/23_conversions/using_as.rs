// A conversão de tipos em Rust é feita através do operador `as`.
// Observe que o operador `as` não é usado apenas na conversão de tipos. Ele também auxilia
// na renomeação de importações.

fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    // TODO: Faça a conversão antes de dividir.
    let values_len = values.len() as f64;
    total / values_len
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}
