#![allow(unused)]
use variant_counter::VariantCount;

struct Info {
    message: String,
    file: String,
    line: u64,
}

#[derive(VariantCount)]
enum Level<'a> {
    #[counter(ignore)]
    Trace,
    Debug {
        line: usize,
    },
    Info(Info),
    #[counter(weight = 5)]
    Warn(),
    #[counter(group = "Fatal", weight = 10)]
    Error(usize),
    Fatal(&'a str),
}

fn main() {
    let mut counter = Level::counter();
    counter.record(&Level::Trace);
    counter.record(&Level::Debug { line: 10 });
    counter.record(&Level::Trace);
    counter.record(&Level::Debug { line: 20 });
    counter.record(&Level::Warn());
    counter.record(&Level::Error(1));
    counter.record(&Level::Fatal("fatal error"));

    assert_eq!(counter.check(&Level::Error(1)), Some(10));
    counter.discard(&Level::Error(1));

    assert_eq!(counter.check(&Level::Trace), None);
    println!("{:?}", &counter.to_map());
    counter.reset();
    println!("{:?}", &counter.to_group_map());
}
