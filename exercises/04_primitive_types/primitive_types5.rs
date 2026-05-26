fn main() {
    let cat = ("Furry McFurson", 3.5);

    // TODO: Desestruture a tupla `cat` em uma única instrução para que o println funcione.
    // let /* your pattern here */ = cat;
    let (name, age) = cat;

    println!("{name} is {age} years old");
}
