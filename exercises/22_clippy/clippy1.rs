// A ferramenta Clippy é uma coleção de verificadores de código para analisar seu código, permitindo que você
// encontre erros comuns e melhore seu código Rust.
// Para estes exercícios, o código não será compilado quando houver avisos do Clippy.
// Verifique as sugestões do Clippy na saída para resolver o exercício.

fn main() {
    // TODO: Corrija o erro de lint do Clippy nesta linha.
    let pi = std::f32::consts::PI;
    let radius: f32 = 5.0;

    let area = pi * radius.powi(2);

    println!("The area of a circle with radius {radius:.2} is {area:.5}");
}
