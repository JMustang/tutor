// Este exercício é uma versão alterada do exercício `errors4`. Ele usa alguns
// conceitos que só abordaremos mais adiante no curso, como `Box` e a
// trait `From`. Não é importante entendê-los em detalhes agora, mas
// você pode ler adiante se quiser. Por enquanto, pense no tipo `Box<dyn ???>` como
// um tipo "Eu quero qualquer coisa que faça ???".
// Resumidamente, este caso de uso específico para boxes é para quando você quer possuir um
// valor e só se importa que ele seja um tipo que implemente uma trait específica.
// Para isso, o `Box` é declarado como do tipo `Box<dyn Trait>`, onde
// `Trait` é a trait que o compilador procura em qualquer valor usado nesse
// contexto. Para este exercício, esse contexto são os erros potenciais que
// podem ser retornados em um `Result`.
use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// Isso é necessário para que `CreationError` possa implementar `Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl Error for CreationError {}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// TODO: Adicionar o tipo de retorno correto `Result<(), Box<dyn Error>>`.
// TODO: O que podemos usar para descrever ambos os erros? Existe alguma trait que ambos os erros implementam?
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}
