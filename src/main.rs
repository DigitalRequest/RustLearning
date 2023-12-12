use std::time::{Duration, Instant};

mod functions;

fn main() {
    let num_instances = 20;
    let mut total_time = Duration::default();

    for _ in 0..num_instances {
        let mut input = Vec::new();

        for _ in 0..100000 {
            let mut word = String::new();
            for _ in 0..50 {
                word.push('a');
                word.push('b');
                word.push('c');
            }
            input.push(word);
        }

        let start_time = Instant::now();
        let result = functions::remove_consec_dup_optimized(input);
        let end_time = Instant::now();

        total_time += end_time - start_time;

        // Imprima apenas uma parte do resultado para evitar uma saída muito longa
        println!("{:?}", &result[0..10]);
        println!("Total de strings no resultado: {}", result.len());
    }

    let average_time = total_time / num_instances as u32;
    println!("Tempo médio de execução: {:?}", average_time);
}