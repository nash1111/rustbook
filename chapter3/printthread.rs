use std::thread;

fn main() {
    let mut handles = Vec::new();

    for x in 0..100 {
        handles.push(
            thread::spawn(move || {
                println!("Hello from a thread! {}", x);
            })
        );
    }

    for handle in handles {
        let _ = handle.join();
    }
}