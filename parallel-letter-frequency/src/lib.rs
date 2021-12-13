use std::collections::HashMap;
use std::thread;
// I can't get it to work faster than the sequential version. I have also tried with an Arc and Mutex.
pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.len() == 0 {
        return HashMap::new();
    }
    let mut result: HashMap<char, usize> = HashMap::new();
    let mut handlers = vec![];
    let chunk_size = (input.len() as f64 / worker_count as f64).ceil();

    input
        .chunks(chunk_size as usize)
        .for_each(|chunk| {
            for _line in chunk {
                let line = _line.to_string();
                let handle = thread::spawn(move || {
                    let mut line_map = HashMap::new();
                    line.to_lowercase()
                        .chars()
                        .filter(|c| c.is_alphabetic())
                        .for_each(|c| {
                            (*line_map.entry(c).or_insert(0)) += 1;
                        });
                    line_map
                });
                handlers.push(handle);
            }
        });

    for handle in handlers {
        let thread_result = handle.join().unwrap();
        for (key, value) in thread_result.iter() {
            (*result.entry(*key).or_insert(0)) += value;
        }
    }
    result

    /*
    // Implementation with messages, even slower.

    let (tx, rx) = mpsc::channel();

    input
        .chunks(worker_count)
        .for_each(|chunk| {
            for i in 0..min(worker_count, chunk.len()) {
                let tx1 = tx.clone();
                let line = String::from(chunk[i]);
                thread::spawn(move || {
                    let mut line_hash = HashMap::new();
                    line
                        .to_lowercase()
                        .chars()
                        .filter(|letter| letter.is_alphabetic())
                        .for_each(|letter| {
                            *line_hash.entry(letter).or_insert(0) += 1;
                        });
                    tx1.send(line_hash).unwrap();
                });
            }
        });
    drop(tx);

    for message in rx {
        for (letter, count) in message.into_iter() {
            *result.entry(letter).or_insert(0) += count;
        }
    } */

    //result
}
