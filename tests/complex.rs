#![allow(unused)]
use variant_counter::*;

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
fn test_complex() {
    type C = ComplexLevel<'static>;

    assert_eq!(C::variant_count(), 6);

    let mut counter = C::counter();
    counter.record(&C::Trace);
    counter.record(&C::Debug { line: 10 });
    counter.record(&C::Info(Info {
        message: "info".into(),
        file: "test.rs".into(),
        line: 1,
    }));
    counter.record(&C::Warn());
    counter.record(&C::Error(1));
    counter.record(&C::Fatal("fatal error"));
}
