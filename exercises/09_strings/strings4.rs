// As chamadas a esta função devem ser substituídas por chamadas a `string_slice` ou `string`.
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: Aqui estão alguns valores - alguns são `String`, outros são `&str`.
// TODO: Sua tarefa é substituir `placeholder(…)` por `string_slice(…)`
// TODO: ou `string(…)` dependendo do que você acha que cada valor representa.

fn main() {
    string_slice("blue");

    string("red".to_string());

    string(String::from("hi"));

    string("rust is fun!".to_owned());

    string("nice weather".into());

    string(format!("Interpolation {}", "Station"));

    // AVISO: Isto é indexação de bytes, não indexação de caracteres.
    // A indexação de caracteres pode ser feita usando `s.chars().nth(INDEX)`.
    string_slice(&String::from("abc")[0..1]);

    string_slice("  hello there ".trim());

    string("Happy Monday!".replace("Mon", "Tues"));

    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
