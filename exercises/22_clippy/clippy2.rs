fn main() {
    let mut res = 42;
    let option = Some(12);
    // TODO: Corrija o problema de acúmulo de lint no Clippy.
    if let Some(x) = option {
        res += x;
    }

    println!("{res}");
}
