// Calcula a potência de 2 usando um deslocamento de bits.
// `1 << n` é equivalente a "2 elevado à potência de n".
fn power_of_2(n: u8) -> u64 {
    1 << n
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        // TODO: Teste a função `power_of_2` com alguns valores.
        assert_eq!(power_of_2(2), 4);
        assert_eq!(power_of_2(4), 16);
        assert_eq!(power_of_2(0), 1);
        assert_eq!(power_of_2(8), 256);
    }
}
