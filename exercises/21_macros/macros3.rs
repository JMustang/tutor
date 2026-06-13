// TODO: Corrija o erro do compilador sem remover a definição da macro deste
// TODO: módulo.
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
    pub(crate) use my_macro;
}

fn main() {
    macros::my_macro!();
}
