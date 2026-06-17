// Isto é semelhante ao exercício anterior com `from_into`. Mas desta vez, vamos
// implementar `FromStr` e retornar erros em vez de recorrer a um valor padrão.
// Além disso, ao implementar `FromStr`, você pode usar o método `parse`
// em strings para gerar um objeto do tipo implementador. Você pode ler
// mais sobre isso na documentação:
// https://doc.rust-lang.org/std/str/trait.FromStr.html

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u8,
}

// Usaremos esse tipo de erro para a implementação de `FromStr`.
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Número incorreto de campos
    BadLen,
    // Empty name field
    NoName,
    // Erro encapsulado de parse::<u8>()
    ParseInt(ParseIntError),
}

// TODO: Complete esta implementação de `FromStr` para poder analisar um `Person`
// de uma string no formato "Mark,20".
// Observe que você precisará analisar o componente de idade em um `u8` com algo
// como `"4".parse::<u8>()`.
// Etapas:
// 1. Divida a string fornecida pelas vírgulas presentes nela.
// 2. Se a operação de divisão retornar menos ou mais de 2 elementos, retorne o
// erro `ParsePersonError::BadLen`.
// 3. Use o primeiro elemento da operação de divisão como o nome.
// 4. Se o nome estiver vazio, retorne o erro `ParsePersonError::NoName`.
// 5. Analise o segundo elemento da operação de divisão em um `u8` como a idade.
// // 6. Se a análise da idade falhar, retorne o erro `ParsePersonError::ParseInt`.
impl FromStr for Person {
    type Err = ParsePersonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Err(ParsePersonError::BadLen);
        }

        let name = parts[0].trim();
        if name.is_empty() {
            return Err(ParsePersonError::NoName);
        }

        let age_str = parts[1].trim();
        let age = age_str.parse::<u8>().map_err(ParsePersonError::ParseInt)?;

        Ok(Person {
            name: name.to_string(),
            age,
        })
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>();
    println!("{p:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use ParsePersonError::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }

    #[test]
    fn missing_age() {
        assert!(matches!("John,".parse::<Person>(), Err(ParseInt(_))));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!("John,twenty".parse::<Person>(), Err(ParseInt(_))));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(",".parse::<Person>(), Err(NoName | ParseInt(_))));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(NoName | ParseInt(_)),
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!("John,32,man".parse::<Person>(), Err(BadLen));
    }
}
