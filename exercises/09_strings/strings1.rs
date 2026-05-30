// TODO: Corrija o erro de compilação sem alterar a assinatura da função.
fn current_favorite_color() -> String {
    let color = "blue";
    color.to_string()
}

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");
}
