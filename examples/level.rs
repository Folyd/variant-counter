#![allow(unused)]
use variant_counter::VariantCount;

#[derive(VariantCount)]
enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

fn main() {
    let level = Level::Debug;
    level.counter();
}
