// A trait `AppendBar` possui apenas uma função que adiciona "Bar" a qualquer objeto
// que implemente essa trait.
trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    // TODO: Implemente `AppendBar` para o tipo `String`.
    // A função 'appendBar' deve retornar uma nova 'String' que é a concatenação da string original com 'Bar'.

    fn append_bar(self) -> Self {
        format!("{}Bar", self)
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {s}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), "FooBar");
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(String::from("").append_bar().append_bar(), "BarBar");
    }
}
