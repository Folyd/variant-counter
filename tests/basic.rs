#![cfg(feature = "full")]
#![allow(unused)]
use variant_counter::*;

#[derive(VariantCount)]
enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[test]
fn test_basic() {
    assert_eq!(Level::variant_count(), 5);

    let mut counter = Level::counter();

    let debug = Level::Debug;
    assert_eq!(counter.check_debug(), 0);

    counter.record(&debug);
    counter.record(&debug);
    assert_eq!(counter.check_debug(), 2);

    counter.erase_debug();
    assert_eq!(counter.check_debug(), 1);

    counter.discard(&debug);
    assert_eq!(counter.check_debug(), 0);

    counter.record(&Level::Info);
    counter.record(&Level::Warn);
    counter.record(&Level::Error);
    counter.reset();
    assert_eq!(counter.check_info(), 0);
    assert_eq!(counter.check_warn(), 0);
    assert_eq!(counter.check_error(), 0);
}
