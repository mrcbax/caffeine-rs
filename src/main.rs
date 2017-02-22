extern crate keystroke;
extern crate chrono;

use std::{thread, time};

use keystroke::{Key, Physical};
use keystroke::{release_key};
use chrono::Duration;

fn main() {
    keep_awake(59);
}

fn reset_timer(counter: Duration, seconds: i64) -> Duration {
    counter + Duration::seconds(seconds)
}

fn send_key_press() {
    use Key::Physical;
    use Physical::Shift;
    release_key(Physical(Shift));
}

fn keep_awake(seconds: i64){
    let sintinel: bool = true;
    let mut counter = reset_timer(Duration::seconds(seconds), 0);
    while sintinel {
        if counter == Duration::seconds(0) {
            counter = reset_timer(counter, seconds);
            send_key_press();
        }
        else {
            println!("{}", counter);
        }
        thread::sleep(time::Duration::from_millis(1000));
        counter = counter - Duration::seconds(1);
    }
}
