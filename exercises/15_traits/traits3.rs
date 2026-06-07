trait Licensed {
    // TODO: Adicionar uma implementação padrão para `licensing_info` para que
    // TODO: implementações como as duas structs abaixo possam compartilhar esse comportamento padrão
    // TODO: sem repetir a função.
    // TODO: As informações de licença padrão devem ser a string "Licença padrão".
    fn licensing_info(&self) -> String {
        "Default license".to_string()
    }
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

impl Licensed for SomeSoftware {} // Don't edit this line.
impl Licensed for OtherSoftware {} // Don't edit this line.

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = "Default license";
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}
