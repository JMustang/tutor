fn main() {
    // TODO: Crie um array chamado `a` com pelo menos 100 elementos.
    // let a = ???
    let a = [0; 100];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
