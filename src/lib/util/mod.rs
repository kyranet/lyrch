pub mod bucket;
pub mod event_handlers;
pub mod macros;
pub mod resolvers;
pub mod logger;

use std::iter::repeat;

pub fn percentage(len: usize, filled: f32) -> String {
    let repeat_fill = (filled * len as f32) as usize;
    let repeat_empty = len - repeat_fill;
    format!(
        "{}{}",
        repeat("█").take(repeat_fill).collect::<String>(),
        repeat(" ").take(repeat_empty).collect::<String>()
    )
}
