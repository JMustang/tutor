// Dando continuidade ao exercício anterior, queremos que todas as threads concluam seu
// trabalho. Mas desta vez, as threads criadas precisam ser responsáveis ​​por atualizar um
// valor compartilhado: `JobStatus.jobs_done`

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // TODO: `Arc` não é suficiente se você deseja um estado compartilhado **mutável**.
    let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }));

    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // TODO: Você precisa realizar uma ação antes de atualizar um valor compartilhado.
            // status_shared.jobs_done += 1;
            let mut data = status_shared.lock().unwrap();
            data.jobs_done += 1;
        });
        handles.push(handle);
    }

    // Aguardando a conclusão de todas as tarefas.
    for handle in handles {
        handle.join().unwrap();
    }

    // TODO: Imprima o valor de `JobStatus.jobs_done`.
    let final_status = status.lock().unwrap();
    println!("Jobs done: {}", final_status.jobs_done);
}
