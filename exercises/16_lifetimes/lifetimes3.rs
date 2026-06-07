// Tempos de vida também são necessários quando structs mantêm referências.

// TODO: Corrigir os erros de compilação relacionados à struct.
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let book = Book {
        author: "George Orwell",
        title: "1984",
    };

    println!("{} by {}", book.title, book.author);
}
