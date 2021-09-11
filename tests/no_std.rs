#![cfg(not(feature = "std"))]

use variant_counter::VariantCount;

#[derive(VariantCount)]
enum Lang {
    #[counter(group = "Non-GC")]
    Rust,
    #[counter(group = "Non-GC")]
    Cpp,
    #[counter(group = "GC")]
    Golang,
    #[counter(group = "GC")]
    Swift,
}

#[test]
fn test_no_std() {
    let mut counter = Lang::counter();
    counter.record(&Lang::Rust);
    counter.record(&Lang::Cpp);
    counter.record(&Lang::Golang);
    counter.record(&Lang::Swift);

    let data = counter.group_aggregate();
    assert_eq!(data[0], ("GC", 2));
    assert_eq!(data[1], ("Non-GC", 2));
}
