// O uso de tipos de erro genéricos como `Box<dyn Error>` não é recomendado para
// código de biblioteca onde os chamadores podem querer tomar decisões com base no erro
// conteúdo em vez de imprimi-lo ou propagá-lo. Aqui, definimos
// um tipo de erro personalizado para permitir que os chamadores decidam o que fazer em seguida
// quando nossa função retornar um erro.

use std::num::ParseIntError;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// Um tipo de erro personalizado que usaremos em `PositiveNozeroInteger::parse`.
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError),
}

impl ParsePosNonzeroError {
    fn from_creation(err: CreationError) -> Self {
        Self::Creation(err)
    }

    // TODO: Adicionar outra função de conversão de erros aqui.
    //TODO: fn from_parse_int(???) -> Self { ??? }
    fn from_parse_int(err: ParseIntError) -> Self {
        Self::ParseInt(err)
    }
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(Self(x as u64)),
        }
    }

    fn parse(s: &str) -> Result<Self, ParsePosNonzeroError> {
        // TODO: alterar isso para retornar um erro apropriado em vez de causar pânico
        //TODO: quando `parse()` retornar um erro.
        let x: i64 = s.parse().map_err(ParsePosNonzeroError::from_parse_int)?;
        Self::new(x).map_err(ParsePosNonzeroError::from_creation)
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        assert!(matches!(
            PositiveNonzeroInteger::parse("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_)),
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            PositiveNonzeroInteger::parse("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative)),
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            PositiveNonzeroInteger::parse("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero)),
        );
    }

    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42).unwrap();
        assert_eq!(x.0, 42);
        assert_eq!(PositiveNonzeroInteger::parse("42"), Ok(x));
    }
}
