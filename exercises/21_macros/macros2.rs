// TODO: Corrija o erro de compilação movendo toda a definição desta macro.
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}
fn main() {
    my_macro!();
}
