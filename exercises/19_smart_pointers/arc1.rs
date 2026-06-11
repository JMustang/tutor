// Neste exercício, recebemos um `Vec` de `u32` chamado `numbers` com valores
// variando de 0 a 99. Gostaríamos de usar este conjunto de números em 8
// threads diferentes simultaneamente. Cada thread obterá a soma de
// cada oitavo valor com um deslocamento.

//
// A primeira thread (deslocamento 0) somará 0, 8, 16, …
// A segunda thread (deslocamento 1) somará 1, 9, 17, …
// A terceira thread (deslocamento 2) somará 2, 10, 18, …
// …
// A oitava thread (deslocamento 7) somará 7, 15, 23, …
//
// Cada thread deve possuir um ponteiro de contagem de referências para o vetor de
// números. Mas `Rc` não é thread-safe. Portanto, precisamos usar `Arc`.

//
// Não se distraia com a forma como as threads são criadas e unidas. Praticaremos isso mais tarde nos exercícios sobre threads.
//

// Don't change the lines below.
#![forbid(unused_imports)]
use std::{sync::Arc, thread};

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();

    // TODO: Defina `shared_numbers` usando `Arc`.
    // let shared_numbers = ???;
    let shared_numbers = Arc::new(numbers);

    let mut join_handles = Vec::new();

    for offset in 0..8 {
        // TODO: Defina `child_numbers` usando `shared_numbers`.
        // let child_numbers = ???;
        let child_numbers = Arc::clone(&shared_numbers);

        let handle = thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {offset} is {sum}");
        });

        join_handles.push(handle);
    }

    for handle in join_handles.into_iter() {
        handle.join().unwrap();
    }
}
