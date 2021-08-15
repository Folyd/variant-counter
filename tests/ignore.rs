#![allow(unused)]
use variant_counter::*;

#[derive(VariantCount)]
enum Level {
    #[counter(ignore)]
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[test]
fn test_ignore() {
    assert_eq!(Level::variant_count(), 5);

    let mut counter = Level::counter();

    let debug = Level::Debug;

    counter.record(&Level::Trace);
}
