use rayon::prelude::*;
use std::{thread, time};
use rand::prelude::*;

fn main() {
    let messages = vec!["one", "two", "three"];
    messages.par_iter().for_each(|m| {
        let mut rng = rand::thread_rng();
        let t = time::Duration::from_secs(rng.gen_range(0..3));
        thread::sleep(t);
        println!("{:?}", m)
    });
}
