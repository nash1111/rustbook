//use std::rc::Rc;
// Rc: 所有権を共有するために、所有権の保持者をカウントする、0になるとメモリ開放
// Arc: Rcのマルチスレッド版
use std::sync::{ Arc, Mutex };
use std::thread;

fn main() {
    let mut handles = Vec::new();
    let data = Arc::new(Mutex::new(vec![1; 10]));

    for x in 0..10 {
        let data_ref = data.clone();
        handles.push(
            thread::spawn(move || {
                let mut data = data_ref.lock().unwrap();
                data[x] += 1;
            })
        );
    }

    for handle in handles {
        let _ = handle.join();
    }

    dbg!(data);
}