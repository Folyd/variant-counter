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

struct Info {
    message: String,
    file: String,
    line: u64,
}

#[derive(VariantCount)]
enum ComplexLevel<'a> {
    Trace,
    Debug { line: usize },
    Info(Info),
    Warn(),
    Error(usize),
    Fatal(&'a str),
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

#[test]
fn test_complex() {
    type C = ComplexLevel<'static>;

    assert_eq!(C::variant_count(), 6);

    let mut counter = C::counter();
    counter.record(&C::Trace);
    counter.record(&C::Debug { line: 10 });
    counter.record(&C::Trace);
    counter.record(&C::Info(Info {
        message: "info".into(),
        file: "test.rs".into(),
        line: 1,
    }));
    counter.record(&C::Warn());
    counter.record(&C::Error(1));
    counter.record(&C::Fatal("fatal error"));
}
