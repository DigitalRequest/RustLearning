use std::time::{Duration, Instant};
use std::collections::HashMap;
use rand::Rng;

mod functions;

fn main() {
    test_fun();
}

fn sum_array_singles(arr: Vec<i32>) -> i32 {
    let mut dict: HashMap<_, _> = HashMap::new();
    let mut sum: i32 = 0;

    for x in &arr {
        let value = if dict.contains_key(&x) {
            2
        } else {
            1
        };

        dict.insert(x, value);
    }

    for x in arr.iter() {
        if let Some(&value) = dict.get(&x) {
            if value == 1 {
                sum += x;
            }
        }
    }

    sum
}

fn test_fun() {
    let num_instances = 20;
    let mut total_time = Duration::default();

    for _ in 0..num_instances {
        let mut input = Vec::new();

        for _ in 0..100000 {
            input.push(rand::thread_rng().gen_range(1..=10000));
        }

        let start_time = Instant::now();
        let result = sum_array_singles(input);
        let end_time = Instant::now();

        total_time += end_time - start_time;

        println!("Result: {}", result)
    }

    let average_time = total_time / num_instances as u32;
    println!("Tempo médio de execução: {:?}", average_time);
}