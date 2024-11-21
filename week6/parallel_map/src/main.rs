use crossbeam_channel;
use std::{thread, time};

fn parallel_map<T, U, F>(mut input_vec: Vec<T>, num_threads: usize, f: F) -> Vec<U>
where
    F: FnOnce(T) -> U + Send + Copy + 'static,
    T: Send + 'static,
    U: Send + 'static + Default,
{
    let mut output_vec: Vec<U> = Vec::with_capacity(input_vec.len());
    let (sender, receiver) = crossbeam_channel::unbounded::<T>();
    let (result_sender, result_receiver) = crossbeam_channel::unbounded::<U>();
    let mut thrs = Vec::new();
    for _ in 0..num_threads {
        let receiver = receiver.clone();
        let result_sender = result_sender.clone();
        let thr = thread::spawn(move || {
            while let Ok(num) = receiver.recv() {
                let result = f(num);
                result_sender.send(result).unwrap();
            }
        });
        thrs.push(thr);
    }
    for num in input_vec.drain(..) {
        sender.send(num).unwrap();
    }
    drop(sender);
    for i in thrs {
        i.join().unwrap();
    }
    drop(result_sender);
    while let Ok(result) = result_receiver.recv() {
        output_vec.push(result);
    }
    output_vec
}

fn main() {
    let v = vec![6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 12, 18, 11, 5, 20];
    let squares = parallel_map(v, 10, |num| {
        println!("{} squared is {}", num, num * num);
        thread::sleep(time::Duration::from_millis(500));
        num * num
    });
    println!("squares: {:?}", squares);
}
