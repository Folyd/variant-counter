#![allow(unused)]
use variant_counter::VariantCount;

#[derive(VariantCount)]
enum Opt<T> {
    Some(T),
    None,
}
fn main() {
    let mut counter = Opt::<usize>::counter();
    counter.record(&Opt::Some(1));
    counter.record(&Opt::<usize>::None);
    counter.erase(&Opt::Some(1));
    counter.erase(&Opt::Some(1));
    assert_eq!(counter.check(&Opt::Some(1)), 0);
    println!("{:?}", counter.to_map());
}
