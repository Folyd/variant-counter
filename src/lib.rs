//! The efficient and elegant crate to count variants of Rust's Enum.
//!
//! ## Get started
//!
//! ### `#[derive(VariantCount)]`
//!
//! ```rust
//! # use variant_counter::VariantCount;
//! #[derive(VariantCount)]
//! pub enum Enum {
//!   Variant1,
//!   Variant2,
//! }
//! ```
//!
//! ### Record your variant
//!
//! ```rust
//! # use variant_counter::VariantCount;
//! # #[derive(VariantCount)]
//! # pub enum Enum {
//! #   Variant1,
//! #   Variant2,
//! # }
//! let mut counter = Enum::counter();
//! counter.record(&Enum::Variant1);
//! ```
//!
//! ### Erase your record with `erase_*()` methods
//!
//! ```rust,ignore
//! counter.erase_variant1();
//! ```
//!
//! Those `erase_*()` methods are under `erase` feature flag, and disabled by default.
//!
//! ### Check your record with `check_*()` methods
//!
//! ```rust,ignore
//! assert_eq!(counter.check_variant1(), 1);
//! ```
//!
//! Those `check_*()` methods are under `check` feature flag, and disabled by default.
//!
//! ### `discard()`, or `reset()` the data
//!
//! ```rust,ignore
//! // Clear the `Enum::Variant1`'s data.
//! counter.discard(&Enum::Variant1);
//!
//! // Clear all variants data.
//! counter.reset();
//! ```
//!
//! ### Ignore a variant
//!
//! ```rust
//! # use variant_counter::VariantCount;
//! #[derive(VariantCount)]
//! pub enum Level {
//!   #[counter(ignore)]
//!   Trace,
//!   Debug,
//!   Info,
//!   Warn,
//!   Error,
//! }
//! ```
//!
//! If a variant was ignored, it has no effect when your record that variant.
//!
//! ```rust,ignore
//! let mut counter = Level::counter();
//! // Record nothing...
//! counter.record(&Level::Trace);
//! ```
//!
//! ### Aggregate your data
//!
//! ```rust,ignore
//! let data = counter.aggregate();
//! ```
//!
//! ### Group variants
//!
//! ```rust
//! # use variant_counter::VariantCount;
//! #[derive(VariantCount)]
//! pub enum Platform {
//!   #[counter(group = "Mobile")]
//!   Android,
//!   #[counter(group = "Mobile")]
//!   IOS,
//!   #[counter(group = "Desktop")]
//!   Windows,
//!   #[counter(group = "Desktop")]
//!   Linux,
//!   #[counter(group = "Desktop")]
//!   MacOS,
//!   #[counter(group = "Desktop")]
//!   ChromeOS,
//!   Others,
//! }
//!
//! let counter = Platform::counter();
//! // Group version of aggregate method
//! let group_data = counter.group_aggregate();
//! ```
//! ### Statistics
//!
//! ```rust,ignore
//! // Sum
//! counter.sum();
//!
//! // Avg
//! counter.avg();
//!
//! // Variance
//! counter.variance();
//!
//! // Standard variance
//! counter.sd();
//! ```
//!
//! ### Weighted
//!
//! ```rust
//! # use variant_counter::VariantCount;
//! #[derive(VariantCount)]
//! enum Rating {
//!   #[counter(weight = 1)]
//!   Hated,
//!   #[counter(weight = 2)]
//!   Disliked,
//!   #[counter(weight = 3)]
//!   Ok,
//!   #[counter(weight = 4)]
//!   Liked,
//!   #[counter(weight = 5)]
//!   Loved,
//! }
//!
//! let mut counter = Rating::counter();
//! counter.record(&Rating::Loved);
//!
//! let w = counter.weighted();
//!
//! // Sum
//! w.sum();
//!
//! // Avg
//! w.avg();
//!
//! // Variance
//! w.variance();
//!
//! // Standard variance
//! w.sd();
//! ```
//!
//! ## Feature flags
//!
//! - `full`
//!
//! - `check`
//!
//! - `erase`
//!
//! - `std`

pub use variant_counter_derived::*;

/// The core `VariantCount` trait which provides an accosiated `counter()` method
/// to get the concrete counter type.
///
/// You should derive the `VariantCount` to auto impl this trait.
///
/// ```rust
/// # use variant_counter::VariantCount;
/// #[derive(VariantCount)]
/// pub enum Enum {
///   Variant1,
///   Variant2,
/// }
///
/// let mut counter = Enum::counter();
/// counter.record(&Enum::Variant1);
/// ```
pub trait VariantCount {
    /// A concrete counter type.
    type Counter;

    /// The accosiated function to get the concrete counter type.
    fn counter() -> Self::Counter;
}
