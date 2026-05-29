struct ColorRegularStruct {
    // TODO: Adicione os campos que o teste `regular_structs` espera.
    // TODO: Quais tipos os campos devem ter? Quais são os valores mínimo e máximo para cores RGB?
    red: u8,
    green: u8,
    blue: u8,
}

struct ColorTupleStruct(
    /* TODO: Add the fields that the test `tuple_structs` expects */
    u8,
    u8,
    u8,
);

#[derive(Debug)]
struct UnitStruct;

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // TODO: Instancie uma struct regular.
        // let green =
        let green = ColorRegularStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instanciar uma estrutura de tupla.
        // let green =
        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instanciar uma estrutura de unidade.
        // let unit_struct =
        let unit_struct = UnitStruct;
        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}
