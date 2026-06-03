// Esta função retorna a quantidade de sorvete restante na geladeira.
// Se for antes das 22:00 (sistema de 24 horas), restam 5 bolas. Às 22:00,
// alguém come tudo, então não sobra sorvete (valor 0). Retorna `None` se
// `hora_do_dia` for maior que 23.
fn maybe_ice_cream(hour_of_day: u16) -> Option<u16> {
    // TODO: Complete o resto da função.
    if hour_of_day < 22 {
        Some(5)
    } else if hour_of_day == 22 {
        Some(0)
    } else if hour_of_day == 23 {
        Some(0)
    } else {
        None
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_value() {
        // TODO: Corrija este teste. Como você obtém o valor contido no
        // Option?
        let ice_creams = maybe_ice_cream(12);

        assert_eq!(ice_creams, Some(5)); // Don't change this line.
    }

    #[test]
    fn check_ice_cream() {
        assert_eq!(maybe_ice_cream(0), Some(5));
        assert_eq!(maybe_ice_cream(9), Some(5));
        assert_eq!(maybe_ice_cream(18), Some(5));
        assert_eq!(maybe_ice_cream(22), Some(0));
        assert_eq!(maybe_ice_cream(23), Some(0));
        assert_eq!(maybe_ice_cream(24), None);
        assert_eq!(maybe_ice_cream(25), None);
    }
}
