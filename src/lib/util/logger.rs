use chrono::prelude::*;

pub fn create_timestamp() -> String {
    let utc = Utc::now();
    format!("[{year:0>4}-{month:0>2}-{day:0>2} {hour:0>2}:{minute:0>2}:{second:0>2}]",
        year = utc.year(),
        month = utc.month(),
        day = utc.day(),
        hour = utc.hour(),
        minute = utc.minute(),
        second = utc.second()
    )
}

#[macro_export]
macro_rules! log {
    ($content:expr) => {
        println!("{} {}", crate::lib::util::logger::create_timestamp(), $content)
    };
    ($content:expr, $($args:tt)+) => {
        crate::log!(format!($content, $($args)+))
    };
}
