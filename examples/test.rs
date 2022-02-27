use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let mut vec = Vec::new();
    let bands = 13;
    let threads = 4;
    for i in 0..bands {
        vec.push(i);
    }
    let data_mutex = Arc::new(Mutex::new(vec));

    let w = 1280;
    let h = 720;
    let mut images = Vec::new();
    let band_h = h / bands;
    println!("{}", band_h);
    for i in 0..bands {
        if i == bands - 1 {
            println!("height from: {} to: {}", i * band_h, h);
            images.push(h-i*band_h);
        } else {
            println!("height from: {} to: {}", i * band_h, (i + 1) * band_h);
            images.push(band_h);
        }
    }

    let images = Arc::new(Mutex::new(images));

    let mut handles = Vec::new();
    for t in 0..threads {
        let data_mutex_clone = Arc::clone(&data_mutex);
        let images_clone = Arc::clone(&images);
        let handle = thread::spawn(move || loop {
            let mut data = data_mutex_clone.lock().unwrap();
            let mut img = images_clone.lock().unwrap();
            if data.len() == 0 {
                break;
            }
            let working_on = data.remove(0);
            img[working_on] = t+working_on;
            println!("T: {}; ind: {}; rem: {:?}", t, working_on, data);
            drop(data);
            drop(img);
            thread::sleep(Duration::from_millis(1));
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }
    println!("{:?}", images);
}
