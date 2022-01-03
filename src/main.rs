use std::{fs::File, io::Write, time::Instant};

fn main() {
    let mut file = File::create("foo").unwrap();
    for _ in 0..10 {
        let t = Instant::now();
        file.write_all("bar".as_bytes()).unwrap();
        file.sync_all().unwrap();
        println!("{}ms", t.elapsed().as_secs_f64() * 1000.0);
    }
}
