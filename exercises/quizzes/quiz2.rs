// Este é um teste para as seguintes seções:
// - Strings
// - Vetores
// - Semântica de movimentação
// - Módulos
// - Enums
//
// Vamos construir uma pequena máquina na forma de uma função. Como entrada, vamos
// fornecer uma lista de strings e comandos. Esses comandos determinam qual ação
// será aplicada à string. Ela pode ser:
// - Converter a string para maiúsculas
// - Remover os espaços em branco da string
// - Adicionar "barra" à string um número específico de vezes
//
// A forma exata disso será:
// - A entrada será um vetor de tuplas de dois elementos,
// onde o primeiro elemento é a string e o segundo é o comando.

// - O elemento de saída será um vetor de strings.

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete a função conforme descrito acima.
    // TODO: pub fn transformer(input: ???) -> ??? { ??? }
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        input
            .into_iter()
            .map(|(string, command)| match command {
                Command::Uppercase => string.to_uppercase(),
                Command::Trim => string.trim().to_string(),
                Command::Append(times) => string + &"bar".repeat(times),
            })
            .collect()
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: O que precisamos importar para ter `transformer` no escopo?
    // TODO: use ???;
    use super::Command;
    use super::my_module::transformer;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
