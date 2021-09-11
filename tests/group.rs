#![cfg(feature = "full")]
#![allow(unused)]
use variant_counter::VariantCount;

#[derive(VariantCount)]
enum Platform {
    #[counter(group = "mobile")]
    Android,
    #[counter(group = "mobile")]
    #[allow(clippy::upper_case_acronyms)]
    IOS,
    #[counter(group = "desktop")]
    Windows,
    #[counter(group = "desktop")]
    Linux,
    #[counter(group = "desktop")]
    MacOS,
    #[counter(group = "desktop")]
    ChromeOS,
    Others,
}

#[derive(VariantCount)]
enum Ball {
    Basketball,
    Volleyball,
    Football,
    #[counter(group = "Football")]
    Soccer,
}

#[test]
fn test_group() {
    assert_eq!(Platform::variant_count(), 7);

    let mut counter = Platform::counter();
    counter.record(&Platform::Android);
    counter.record(&Platform::Android);
    counter.record(&Platform::Windows);
    counter.record(&Platform::IOS);
    counter.record(&Platform::Others);
    assert_eq!(counter.check_android(), 2);

    let mut map = counter.aggregate();
    assert_eq!(map.len(), 7);
    assert_eq!(
        (
            map.remove("Android"),
            map.remove("IOS"),
            map.remove("Windows"),
            map.remove("Linux"),
            map.remove("MacOS"),
            map.remove("ChromeOS"),
            map.remove("Others"),
        ),
        (
            Some(2),
            Some(1),
            Some(1),
            Some(0),
            Some(0),
            Some(0),
            Some(1),
        )
    );

    let mut group_map = counter.group_aggregate();
    assert_eq!(group_map.len(), 3);
    assert_eq!(
        (
            group_map.remove("mobile"),
            group_map.remove("desktop"),
            group_map.remove("Others")
        ),
        (Some(3), Some(1), Some(1),)
    );
}

#[test]
fn test_group_alias() {
    assert_eq!(Ball::variant_count(), 4);

    let mut counter = Ball::counter();
    counter.record(&Ball::Basketball);
    counter.record(&Ball::Volleyball);
    counter.record(&Ball::Football);
    counter.record(&Ball::Soccer);

    assert_eq!(counter.check_football(), 1);
    assert_eq!(counter.check_soccer(), 1);

    let mut group_map = counter.group_aggregate();
    assert_eq!(group_map.len(), 3);
    assert_eq!(
        (
            group_map.remove("Basketball"),
            group_map.remove("Volleyball"),
            group_map.remove("Football")
        ),
        (Some(1), Some(1), Some(2),)
    );
}
