// O trait `From` é usado para conversões de valor para valor. Se `From` for
// implementado, uma implementação de `Into` será fornecida automaticamente.
// Você pode ler mais sobre isso na documentação:
// https://doc.rust-lang.org/std/convert/trait.From.html
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// Implementamos o trait Default para usá-lo como alternativa quando a string fornecida
// não for conversível em um objeto `Person`.
impl Default for Person {
    fn default() -> Self {
        Self {
            name: String::from("John"),
            age: 30,
        }
    }
}

// TODO: Complete esta implementação de `From` para poder analisar um `Person`
// de uma string no formato "Mark,20".
// Observe que você precisará analisar o componente de idade em um `u8` com algo
// como `"4".parse::<u8>()`.
// Etapas:
// 1. Divida a string fornecida pelas vírgulas presentes nela.
// 2. Se a operação de divisão retornar menos ou mais de 2 elementos, retorne o
// valor padrão de `Person`.
// 3. Use o primeiro elemento da operação de divisão como o nome.
// 4. Se o nome estiver vazio, retorne o valor padrão de `Person`.
// 5. Analise o segundo elemento da operação de divisão em um `u8` como a idade.
// 6. Se a análise da idade falhar, retorne o valor padrão de `Person`.
impl From<&str> for Person {
    fn from(s: &str) -> Self {
        let parts: Vec<_> = s.split(',').collect();
        if parts.len() != 2 {
            return Person::default();
        }
        let name = parts[0].trim();
        if name.is_empty() {
            return Person::default();
        }
        let age = match parts[1].trim().parse::<u8>() {
            Ok(age) => age,
            Err(_) => return Person::default(),
        };
        Person {
            name: name.to_string(),
            age,
        }
    }
}

fn main() {
    // Use a função `from`.
    let p1 = Person::from("Mark,20");
    println!("{p1:?}");

    // Como `From` está implementado para Person, podemos usar `Into`.
    let p2: Person = "Gerald,70".into();
    println!("{p2:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }

    #[test]
    fn test_bad_convert() {
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_good_convert() {
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }

    #[test]
    fn test_bad_age() {
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,dog");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
