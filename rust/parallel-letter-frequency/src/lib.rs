use std::cmp::max;
use std::collections::HashMap;
use std::sync::{mpsc, Mutex};
use std::thread;

fn count_chars(line: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    for c in line.chars().filter(|c| c.is_alphabetic()) {
        counts
            .entry(c.to_ascii_lowercase())
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
    counts
}

pub fn frequency_lines(input: &[&str]) -> HashMap<char, usize> {
    let mut total_counts = HashMap::new();
    for &line in input {
        for (key, value) in count_chars(line) {
            total_counts
                .entry(key)
                .and_modify(|e| *e += value)
                .or_insert(value);
        }
    }
    total_counts
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    thread::scope(|scope| {
        let mut handles = vec![];
        for chunk in input.chunks(max(input.len() / worker_count, 1)) {
            let handle = scope.spawn(move || {
                let counts: HashMap<char, usize> = frequency_lines(chunk);
                counts
            });
            handles.push(handle);
        }

        let mut total_counts: HashMap<char, usize> = HashMap::new();
        for handle in handles {
            let counts = handle.join().unwrap();
            for (key, value) in counts {
                total_counts
                    .entry(key)
                    .and_modify(|e| *e += value)
                    .or_insert(value);
            }
        }
        total_counts
    })
}

//

//

//

// let chunks: &[&str] = input.chunks(worker_count).collect();

// for chunk in input.chunks(input.len() / worker_count) {
//     let handle = thread::spawn(move || {
//         let counts = frequency_lines(chunk);
//     });
//     handle.join().unwrap();
// }
//HashMap::new()

// let mut total_counts = HashMap::new();
// for &line in input {
//     for (key, value) in count_chars(line) {
//         total_counts
//             .entry(key)
//             .and_modify(|e| *e += value)
//             .or_insert(value);
//     }
// }
// total_counts
