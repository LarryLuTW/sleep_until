use chrono::prelude::*;
use chrono::Duration;
use chrono::Local;
use chrono::NaiveTime;
use std::env;
use std::process;
use std::thread;

fn get_curr_time() -> NaiveTime {
    let now = Local::now();
    return NaiveTime::from_hms(now.hour(), now.minute(), now.second());
}

fn get_wakeup_time() -> NaiveTime {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("usage: sleep_until 20:15:00");
        process::exit(1);
    }

    // args[1] = "20:01:00"
    return NaiveTime::parse_from_str(&args[1], "%H:%M:%S").unwrap();
}

fn print_curr_time() {
    println!("It's {} now", get_curr_time());
}

fn raw_sleep(dur: Duration) {
    thread::sleep(dur.to_std().unwrap());
}

fn main() {
    let begin = get_curr_time();
    let end = get_wakeup_time();
    println!("Sleep from {} to {}", begin, end);

    let short_dur = Duration::seconds(10);
    loop {
        print_curr_time();

        let begin = get_curr_time();
        let dur = end.signed_duration_since(begin);

        if dur < Duration::zero() {
            // wake up immediately
            break;
        }

        if dur < short_dur {
            // the last sleep
            raw_sleep(dur);
            break;
        }

        raw_sleep(short_dur);
    }
    print_curr_time();
}
