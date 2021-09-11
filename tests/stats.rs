#![cfg(feature = "full")]
#![allow(unused)]
use variant_counter::*;

#[derive(VariantCount)]
enum Rating {
    #[counter(weight = 1)]
    Hated,
    #[counter(weight = 2)]
    Disliked,
    #[counter(weight = 3)]
    Ok,
    #[counter(weight = 4)]
    Liked,
    #[counter(weight = 5)]
    Loved,
}

#[test]
fn test_stats() {
    let mut counter = Rating::counter();
    counter.record(&Rating::Disliked);
    counter.record(&Rating::Liked);
    counter.record(&Rating::Liked);
    counter.record(&Rating::Liked);
    counter.record(&Rating::Liked);
    counter.record(&Rating::Ok);
    counter.record(&Rating::Ok);
    counter.record(&Rating::Ok);
    counter.record(&Rating::Ok);
    counter.record(&Rating::Ok);
    counter.record(&Rating::Liked);
    counter.record(&Rating::Loved);
    counter.record(&Rating::Hated);
    counter.record(&Rating::Disliked);

    assert_eq!(14, counter.sum());
    assert_eq!(2.8f64, counter.avg());
    assert_eq!(3.3599999999999994, counter.variance());
    assert_eq!(1.8330302779823358, counter.sd());

    let weighted = counter.weighted();
    assert_eq!(15, weighted.total_weight());
    assert_eq!(45, weighted.sum());
    assert_eq!(3.0, weighted.avg());
    assert_eq!(29.466666666666665, weighted.variance());
    assert_eq!(5.428320796219275, weighted.sd());
}
