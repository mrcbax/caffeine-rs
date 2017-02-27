// Inivisible backgrounding for Windows based systems.
#![feature(windows_subsystem)]
#![windows_subsystem = "windows"]

#![feature(type_ascription)]
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

#[derive(Debug, Deserialize)]
struct Config {
    refresh_rate: Option<i64>,
    start_after: Option<i64>,
    exit_after: Option<i64>,
    active_for: Option<i64>,
}

fn main() {
    let config: Config = read_config();
    let refresh_rate: i64 = config.refresh_rate.unwrap();
    let start_after: i64 = config.start_after.unwrap();
    let exit_after: i64 = config.exit_after.unwrap();
    let active_for: i64 = config.active_for.unwrap();
    keep_awake(refresh_rate, start_after, exit_after, active_for);
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
fn keep_awake(refresh_rate: i64, start_after: i64, exit_after: i64, active_for: i64){
    thread::sleep(time::Duration::new(start_after as u64, 0));
    let mut exit: i64 = -1;
    let mut active: i64 = -1;
    if exit_after > 0 {
        exit = exit_after;
    }
    if active_for > 0 {
        active = active_for;
    }
    let mut sintinel: bool = true;
    let mut counter = reset_timer(Duration::seconds(refresh_rate), 0);
    while sintinel {
        if counter == Duration::seconds(0) {
            counter = reset_timer(counter, refresh_rate);
            platform::platform::send_key_press();
            println!("sent keys.");
        }
        thread::sleep(time::Duration::from_millis(1000));
        println!("{} seconds remaining", counter);
        counter = counter - Duration::seconds(1);
        exit -= 1;
        active -= 1;
        if exit == 0 || active == 0 {
            sintinel = false;
        }
    }
    if active >= 0 {
        thread::sleep(time::Duration::new(active as u64, 0));
    }
}


/// Reads a configuration file if it exists.
///
/// Passes along the configuration data as a struct, returns defaults if no config is specified.
fn read_config() -> Config {
    let name = String::from("caffeine.toml");
    let mut decoded: Config = Config { refresh_rate: Some(59), start_after: Some(0), exit_after: Some(-1), active_for: Some(-1)};
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

#[cfg(test)]
mod tests {

    #[test]
    fn path_exists_ls() {
        use path_exists;
        assert!(path_exists("/bin/ls"))
    }

    #[test]
    fn reset_timer_once() {
        use reset_timer;
        assert!(chrono::Duration.seconds(5).eq(reset_timer(chrono::Duration.seconds(0),5)))
    }

}
