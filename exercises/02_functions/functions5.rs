// TODO: Fix the function body without changing the signature.
fn square(num: i32) -> i32 {
    num * num
}

fn main() {
    let answer = square(3);
    println!("The square of 3 is {answer}");
}

/*
    Em Rust, a última expressão de uma função (sem ponto e vírgula)
    é retornada implicitamente. Quando você coloca ; após num * num,
    você transforma aquela expressão em uma declaração (statement),
    que retorna () (a tupla vazia, também chamada de "unit").

    Como a função declarou retorno -> i32, mas o corpo efetivamente retorna (),
    o compilador aponta um erro de tipo:
*/
