use chrono::prelude::*;
use chrono::Duration;
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

    println!("Sleep from {} to {}", begin, end);

    let a_min = Duration::minutes(1);
    loop {
        let now = Local::now();
        let begin = NaiveTime::from_hms(now.hour(), now.minute(), now.second());
        println!("It's {}:{}:{} now", now.hour(), now.minute(), now.second());

        let dur = end.signed_duration_since(begin);
        if dur < a_min {
            thread::sleep(dur.to_std().unwrap());
            break;
        }

        thread::sleep(a_min.to_std().unwrap());
    }
    let now = Local::now();
    println!("It's {}:{}:{} now", now.hour(), now.minute(), now.second());
}
