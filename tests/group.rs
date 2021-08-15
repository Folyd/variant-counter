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

    let mut map = counter.to_map();
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

    let mut group_map = counter.to_group_map();
    assert_eq!(group_map.len(), 3);
    assert_eq!(
        (
            group_map.remove("mobile"),
            group_map.remove("desktop"),
            group_map.remove("Others")
        ),
        (Some(3), Some(1), Some(1),)
    )
}
