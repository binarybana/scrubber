use std::fs::File;
use std::io::prelude::*;

use std::time::{Duration, Instant};
use std::thread::sleep;

use std::sync::{Arc, Mutex};
use std::thread;

use rand::prelude::*;
use rand::distributions::Alphanumeric;

fn main() -> std::io::Result<()> {
    let mut buf = Vec::with_capacity(1024*1024*1024);
    let mut rng = rand::thread_rng();
    let bytes_written = Arc::new(Mutex::new(0));
    let bytes1 = bytes_written.clone();
    thread::spawn(move || {
        let now = Instant::now();
        loop {
            sleep(Duration::from_millis(1000));
            {
                let bytes = bytes1.lock().unwrap();
                println!("{} bytes {} secs {} MB/sec", *bytes, now.elapsed().as_secs(), *bytes as f64/now.elapsed().as_secs() as f64/1024.0/1024.0);
            }
        }
    });

    let mut count = 0;

    std::fs::create_dir("files")?;
    loop {
        let rand_string: String = rng
            .sample_iter(&Alphanumeric)
            .take(30)
            .collect();
        let rand_string: String = String::from("files/") + &rand_string;
        let mut file = File::create(rand_string)?;
        buf.resize_with(rng.gen_range(10, 1024*1024*100), || rng.gen());
        // buf.resize(rng.gen_range(10, 1024*1024*100), 0u8);
        // rng.fill(&mut buf[..]);
        {
            let mut bytes2 = bytes_written.lock().unwrap();
            *bytes2 += buf.len();
        }
        file.write_all(&buf)?;

        count += 1;
        if count > 10 {
            std::process::Command::new("rm")
                .arg("-rf")
                .arg("files")
                .output()
                .expect("failed to execute process");
            std::fs::create_dir("files")?;
            count = 0;
        }

    }
}
