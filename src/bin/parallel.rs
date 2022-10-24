//
// Rayon is super easy! I don't think I need to even mess around with tokio yet
// because all I'm trying to do is roughly replicate Go's concurrency model.
//
// Using Rayon, I can use `par_iter().for_each(_)` to do something for each thread.
// This probably makes it even easier than go because I should be able to `collect()`
// all the results from the tasks, such as stdout/stderr.
//

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
