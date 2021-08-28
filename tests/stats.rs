#![allow(unused)]
use variant_counter::*;

#[derive(VariantCount)]
enum Rate {
    Hated,
    Disliked,
    Ok,
    Liked,
    Loved,
}

#[test]
fn test_stats() {
    let mut counter = Rate::counter();
    counter.record(&Rate::Disliked);
    counter.record(&Rate::Liked);
    counter.record(&Rate::Liked);
    counter.record(&Rate::Liked);
    counter.record(&Rate::Liked);
    counter.record(&Rate::Ok);
    counter.record(&Rate::Ok);
    counter.record(&Rate::Ok);
    counter.record(&Rate::Ok);
    counter.record(&Rate::Ok);
    counter.record(&Rate::Liked);
    counter.record(&Rate::Loved);
    counter.record(&Rate::Hated);
    counter.record(&Rate::Disliked);

    assert_eq!(14, counter.sum());
    assert_eq!(2.8f64, counter.avg());
    assert_eq!(3.3599999999999994, counter.variance());
    assert_eq!(1.8330302779823358, counter.sd());
}
