// Este programa cria várias threads, cada uma executada por pelo menos 250 ms, e
// cada thread retorna o tempo que levou para ser concluída. O programa deve
// esperar até que todas as threads criadas tenham terminado e coletar seus
// valores de retorno em um vetor.

use std::{
    thread,
    time::{Duration, Instant},
};

fn main() {
    let mut handles = Vec::new();
    for i in 0..10 {
        let handle = thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("Thread {i} done");
            start.elapsed().as_millis()
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        // TODO: Coletar os resultados de todas as threads no vetor `results`.
        // TODO: Usar a struct `JoinHandle` retornada por `thread::spawn`.
        results.push(handle.join().unwrap());
    }

    if results.len() != 10 {
        panic!("Oh no! Some thread isn't done yet!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("Thread {i} took {result}ms");
    }
}
