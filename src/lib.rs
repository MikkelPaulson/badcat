use std::io::stdin;
use std::io::BufRead;
use std::sync::{Arc, Mutex};

use rayon::prelude::*;

pub fn badcat() {
    let lock: Arc<Mutex<()>> = Arc::default();

    rayon::scope(|s| {
        for line in stdin().lock().lines().map(|l| l.unwrap()) {
            let inner_lock = Arc::clone(&lock);
            s.spawn(move |_| {
                if let Ok(_lock) = inner_lock.lock() {
                    badprint(&line);
                    println!();
                }
            });
        }
    });
}

pub fn badprint(line: &str) {
    line.chars().par_bridge().for_each(|c| print!("{}", c));
}
