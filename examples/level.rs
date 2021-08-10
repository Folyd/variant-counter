#![allow(unused)]
use variant_counter::VariantCount;

#[derive(VariantCount)]
enum Level {
    Trace,
    Debug,
    Info,
    Warn(),
    Error(usize),
}

fn main() {
    let level = Level::Debug;
    let mut counter = level.counter();
    counter.record(&Level::Debug);
    counter.record(&Level::Info);
    counter.record(&Level::Debug);
    counter.record(&Level::Warn());
    counter.record(&Level::Error(1));
    println!("{:?}", &counter);
}
