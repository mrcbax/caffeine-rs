// Inivisible backgrounding for Windows based systems.
#![feature(windows_subsystem)]
#![feature(type_ascription)]
#![windows_subsystem = "windows"]
#![deny(warnings)]

extern crate chrono;
extern crate keystroke;
extern crate toml;

#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::io::prelude::*;
use std::{thread, time};

use chrono::Duration;
use keystroke::{Key, Physical};
use keystroke::{release_key};

fn main() {
    keep_awake(read_config().refresh_rate.unwrap());
}


/// This is what we're going to decode into. Each field is optional, meaning
/// that it doesn't have to be present in TOML.
#[derive(Debug, Deserialize)]
struct Config {
    refresh_rate: Option<i64>,
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

fn read_config() -> Config {
    let mut input = String::new();
    let name = String::from("caffeine.toml");
    File::open(&name).and_then(|mut f| {
        f.read_to_string(&mut input)
    }).unwrap();

    let some_duration: Option<i64> = Some(59);
    let mut decoded: Config = Config { refresh_rate: some_duration };
    decoded: Config = toml::from_str(input.as_str()).unwrap();
    decoded
}
