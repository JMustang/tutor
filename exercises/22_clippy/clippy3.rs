// Aqui estão mais algumas correções fáceis para o Clippy para que você possa ver sua utilidade 📎
// TODO: Corrigir todos os erros de lint do Clippy.

#[rustfmt::skip]
#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<&str> = None;
    // Suponha que você não saiba o valor de `my_option`.
    // No caso de `Some`, queremos imprimir seu valor.
    if my_option.is_some() {
        println!("{}", my_option.unwrap_or("my_option é None!"));
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {my_arr:?}");

    let my_empty_vec: Vec<i32> = vec![];
    println!("This Vec is empty, see? {my_empty_vec:?}");

    let mut value_a = 45;
    let mut value_b = 66;
    // Vamos trocar esses dois!
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {value_a}; value b: {value_b}");
}
