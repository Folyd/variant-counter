#![allow(unused)]
use variant_counter::*;

#[derive(VariantCount)]
enum Level {
    Trace,
    Debug,
    #[counter(weight = 2)]
    Info,
    #[counter(weight = 5)]
    Warn,
    #[counter(weight = 10)]
    Error,
}

#[test]
fn test_weight() {
    assert_eq!(Level::variant_count(), 5);

    let mut counter = Level::counter();

    let debug = Level::Debug;
    assert_eq!(counter.check_debug(), 0);

    counter.record(&debug);
    counter.record(&debug);
    counter.erase_debug();
    assert_eq!(counter.check_debug(), 1);

    counter.record(&Level::Info);
    assert_eq!(counter.check_info(), 1);
    counter.erase_info();
    assert_eq!(counter.check_info(), 0);

    counter.record(&Level::Warn);
    counter.record(&Level::Error);
    assert_eq!(counter.check_warn(), 1);
    assert_eq!(counter.check_error(), 1);

    let weighted = counter.weighted();
    assert_eq!(weighted.check_warn(), 5);
    assert_eq!(weighted.check_error(), 10);
}
