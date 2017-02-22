// Inivisible backgrounding for Windows based systems.
#![feature(windows_subsystem)]
#![feature(type_ascription)]
#![windows_subsystem = "windows"]
#![deny(warnings)]

extern crate chrono;
#[macro_use]
extern crate serde_derive;
extern crate toml;

pub mod platform;

use std::fs;
use std::io::prelude::*;
use std::{thread, time};
use std::path::Path;

use chrono::Duration;

fn main() {
    keep_awake(read_config().refresh_rate.unwrap());
}

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
            platform::platform::send_key_press();
        }
        thread::sleep(time::Duration::from_millis(1000));
        counter = counter - Duration::seconds(1);
    }
}


/// Reads a configuration file if it exists.
///
/// Passes along the configuration data as a struct, returns defaults if no config is specified.
fn read_config() -> Config {
    let name = String::from("caffeine.toml");
    let some_duration: Option<i64> = Some(59);
    let mut decoded: Config = Config { refresh_rate: some_duration };
    if path_exists(&name) {
        let mut input = String::new();
        fs::File::open(&name).and_then(|mut f| {
            f.read_to_string(&mut input)
        }).unwrap();
        decoded: Config = toml::from_str(input.as_str()).unwrap();
    }
    decoded
}
/// Determines if a path exists or not.
///
/// #Arguments
///
/// * `path` - A str reference to a path string.
fn path_exists(path: &str) -> bool {
    Path::new(path).exists()
}
