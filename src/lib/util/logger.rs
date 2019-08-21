use chrono::prelude::*;

pub fn create_timestamp() -> String {
    let utc = Utc::now();
    format!(
        "[{year:0>4}-{month:0>2}-{day:0>2} {hour:0>2}:{minute:0>2}:{second:0>2}]",
        year = utc.year(),
        month = utc.month(),
        day = utc.day(),
        hour = utc.hour(),
        minute = utc.minute(),
        second = utc.second()
    )
}

#[macro_export]
macro_rules! wrap {
    ($opening:expr, $closing:expr) => {
        concat!("\u{001B}[", $opening, "m{}\u{001B}[", $closing, "m")
    };
}

#[macro_export]
macro_rules! debug {
    ($content:expr) => {
        println!(
            concat!(crate::wrap!("35", "39"), " ", "{}"),
            crate::lib::util::logger::create_timestamp(), $content
        )
    };
    ($content:expr, $($args:tt)+) => {
        crate::debug!(format!($content, $($args)+))
    };
}

#[macro_export]
macro_rules! error {
    ($content:expr) => {
        eprintln!(
            concat!(crate::wrap!("97;41", "39;49"), " ", "{}"),
            crate::lib::util::logger::create_timestamp(), $content
        )
    };
    ($content:expr, $($args:tt)+) => {
        crate::error!(format!($content, $($args)+))
    };
}

#[macro_export]
macro_rules! info {
    ($content:expr) => {
        println!(
            concat!(crate::wrap!("93", "39"), " ", crate::wrap!("90", "39")),
            crate::lib::util::logger::create_timestamp(), $content
        )
    };
    ($content:expr, $($args:tt)+) => {
        crate::info!(format!($content, $($args)+))
    };
}

#[macro_export]
macro_rules! log {
    ($content:expr) => {
        println!(
            concat!(crate::wrap!("94", "39"), " ", "{}"),
            crate::lib::util::logger::create_timestamp(), $content
        )
    };
    ($content:expr, $($args:tt)+) => {
        crate::log!(format!($content, $($args)+))
    };
}

#[macro_export]
macro_rules! verbose {
    ($content:expr) => {
        println!(
            concat!(crate::wrap!("90", "39"), " ", crate::wrap!("90", "39")),
            crate::lib::util::logger::create_timestamp(), $content
        )
    };
    ($content:expr, $($args:tt)+) => {
        crate::verbose!(format!($content, $($args)+))
    };
}

#[macro_export]
macro_rules! warn {
    ($content:expr) => {
        eprintln!(
            concat!(crate::wrap!("93", "39"), " ", crate::wrap!("93", "39")),
            crate::lib::util::logger::create_timestamp(), $content
        )
    };
    ($content:expr, $($args:tt)+) => {
        crate::warn!(format!($content, $($args)+))
    };
}

#[macro_export]
macro_rules! wtf {
    ($content:expr) => {
        eprintln!(
            concat!(crate::wrap!("97;41", "39;49"), " ", crate::wrap!("31", "39")),
            crate::lib::util::logger::create_timestamp(), $content
        )
    };
    ($content:expr, $($args:tt)+) => {
        crate::wtf!(format!($content, $($args)+))
    };
}
