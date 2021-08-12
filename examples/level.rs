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
    Warn(),
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

    assert_eq!(counter.check(&Level::Trace), None);
    println!("{:?}", &counter.to_map());
}
