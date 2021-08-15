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

fn main() {
    let mut counter = Platform::counter();
    counter.record(&Platform::Android);
    counter.record(&Platform::Android);
    counter.record(&Platform::Windows);
    counter.record(&Platform::IOS);
    counter.record(&Platform::Others);
    assert_eq!(counter.check_android(), 2);
    println!("{:?}", counter.to_map());
    println!("{:?}", counter.to_group_map());
}
