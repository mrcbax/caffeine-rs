// Inivisible backgrounding for Windows based systems.
#![feature(windows_subsystem)]
#![windows_subsystem = "windows"]

extern crate keystroke;
extern crate chrono;

use std::{thread, time};

use keystroke::{Key, Physical};
use keystroke::{release_key};
use chrono::Duration;

fn main() {
    keep_awake(59);
}

///Resets the timer duration to a given value.
///
/// # Arguments
///
/// * `counter` - A `chrono` crate `Duration`.
/// * `seconds` - An i64 for the number of seconds to add to the duration.
fn reset_timer(counter: Duration, seconds: i64) -> Duration {
    counter + Duration::seconds(seconds)
}

///Sends a key-release action to the OS.
fn send_key_press() {
    use Key::Physical;
    use Physical::Shift;
    release_key(Physical(Shift));
}

///Loop manager.
///
/// #Arguments
///
/// * `seconds` - The number of seconds to pass to `reset-timer`.
fn keep_awake(seconds: i64){
    let sintinel: bool = true;
    let mut counter = reset_timer(Duration::seconds(seconds), 0);
    while sintinel {
        if counter == Duration::seconds(0) {
            counter = reset_timer(counter, seconds);
            send_key_press();
        }
        thread::sleep(time::Duration::from_millis(1000));
        counter = counter - Duration::seconds(1);
    }
}
