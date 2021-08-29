#![cfg(feature = "full")]
use variant_counter::*;

#[derive(VariantCount)]
enum Opt<T> {
    Some(T),
    None,
}

#[derive(VariantCount)]
enum Either<A, B> {
    A(A),
    B(B),
}

#[derive(VariantCount)]
enum Text<'a, T: Sized> {
    Str(&'a str),
    String(String),
    RichText(T),
}

#[test]
fn test_opt() {
    assert_eq!(Opt::<usize>::variant_count(), 2);

    let mut counter = Opt::<usize>::counter();
    let opt_some = Opt::<usize>::Some(1);
    let opt_none = Opt::<usize>::None;

    assert_eq!(counter.check_some(), 0);
    assert_eq!(counter.check_none(), 0);

    counter.record(&opt_some);
    counter.record(&opt_none);

    assert_eq!(counter.check_some(), 1);
    assert_eq!(counter.check_none(), 1);
}

#[test]
fn test_either() {
    type E = Either<bool, usize>;
    assert_eq!(E::variant_count(), 2);

    let mut counter = E::counter();
    assert_eq!(counter.check_a(), 0);
    assert_eq!(counter.check_b(), 0);

    counter.record(&E::A(true));
    counter.record(&E::B(1));

    assert_eq!(counter.check_a(), 1);
    assert_eq!(counter.check_b(), 1);
}

#[test]
fn test_text() {
    type T = Text<'static, Vec<u8>>;
    assert_eq!(T::variant_count(), 3);

    let mut counter = T::counter();
    counter.record(&T::Str("rust"));
    counter.record(&T::String(String::from("hello world")));
    counter.record(&T::RichText(Vec::from("<p>Rust</p>")));

    assert_eq!(counter.check_str(), 1);
}
