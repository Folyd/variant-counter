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
    println!("{:?}", counter.to_map());
}
