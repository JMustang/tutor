// TODO: Esta função se recusa a gerar texto para ser impresso em uma etiqueta de identificação se
// TODO: você passar uma string vazia. Seria melhor se ela explicasse qual é o problema
// TODO: em vez de apenas retornar `None`. Felizmente, Rust tem uma construção similar
// TODO: a `Option` que pode ser usada para expressar condições de erro. Altere
// TODO: a assinatura e o corpo da função para retornar `Result<String, String>` em vez de
// TODO: `Option<String>`.
fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        // Nomes vazios não são permitidos.
        Err("Empty names aren't allowed".to_string())
    } else {
        Ok(format!("Hi! My name is {name}"))
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".to_string()).as_deref(),
            Ok("Hi! My name is Beyoncé"),
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text(String::new())
                .as_ref()
                .map_err(|e| e.as_str()),
            Err("Empty names aren't allowed"),
        );
    }
}
