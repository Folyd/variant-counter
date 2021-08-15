#![allow(unused)]
use variant_counter::VariantCount;

#[derive(VariantCount)]
enum Opt<T> {
    Some(T),
    None,
}
fn main() {
    assert_eq!(Opt::<usize>::variant_count(), 2);
    let mut counter = Opt::<usize>::counter();
    counter.record(&Opt::Some(1));
    counter.record(&Opt::<usize>::None);
    counter.erase_some();
    counter.erase_some();
    assert_eq!(counter.check_some(), 0);
    println!("{:?}", counter.to_map());
}
