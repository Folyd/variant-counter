#![allow(unused)]
use variant_counter::VariantCount;

#[derive(VariantCount)]
enum Platform {
    #[counter(group = "Mobile")]
    Android,
    #[counter(group = "Mobile")]
    #[allow(clippy::upper_case_acronyms)]
    IOS,
    #[counter(group = "Desktop")]
    Windows,
    #[counter(group = "Desktop")]
    Linux,
    #[counter(group = "Desktop")]
    MacOS,
    #[counter(group = "Desktop", weight = 3)]
    ChromeOS,
    Others,
}

fn main() {
    let mut counter = Platform::counter();
    counter.record(&Platform::Android);
    counter.record(&Platform::Android);
    counter.record(&Platform::Windows);
    counter.record(&Platform::IOS);
    counter.record(&Platform::Others);
    #[cfg(feature = "check")]
    assert_eq!(counter.check_android(), 2);
    println!("{:?}", counter.aggregate());
    println!("{:?}", counter.group_aggregate());
}
