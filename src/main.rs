// from https://marabos.nl/atomics/atomics.html

use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use std::thread;
use rand::{thread_rng, Rng};

fn process_item(_num: usize) {
    let mut rng = thread_rng();
    let millis: u64 = rng.gen_range(10..500);
    
    thread::sleep(Duration::from_millis(millis));
}

fn main() {
    let num_done = AtomicUsize::new(0);

    thread::scope(|s| {
        // A background thread to process all 100 items.
        s.spawn(|| {
            for i in 0..100 {
                process_item(i); // Take some time to process this item
                num_done.store(i + 1, Ordering::Relaxed);
            }
        });
        
        let mut prev_num = 0;
        
        // The main thread shows status updates when the count has changed
        loop {
            let current_num = num_done.load(Ordering::Relaxed);
            
            if current_num == 100 {
                break;
            }
            
            if current_num != prev_num {
                prev_num = current_num;
                println!("Working.. {current_num}/100 done");
            }
            
            thread::sleep(Duration::from_millis(1));
        }
    });

    println!("Done!");
}