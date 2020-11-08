use chrono::prelude::*;
use chrono::Local;
use chrono::NaiveTime;
use std::env;
use std::thread;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("usage: sleep_until 20:15:00");
        return;
    }

    let end_str = &args[1]; // "20:01:00"
    let end = NaiveTime::parse_from_str(&end_str, "%H:%M:%S").unwrap();

    let now = Local::now();
    let begin = NaiveTime::from_hms(now.hour(), now.minute(), now.second());

    println!("From {} to {}", begin, end);

    let dur = end.signed_duration_since(begin).to_std().unwrap();
    thread::sleep(dur);
}
