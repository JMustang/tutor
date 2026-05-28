fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // Array

    // TODO: CCrie um vetor chamado `v` que contenha exatamente os mesmos elementos do array `a`.
    // Use the vector macro.
    // let v = ???;
    let v = vec![10, 20, 30, 40];

    (a, v)
}

fn main() {
    // You can optionally experiment here.
    array_and_vec();
    println!(
        "Array and vector created {:?} and {:?}",
        array_and_vec().0,
        array_and_vec().1
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, *v);
    }
}
