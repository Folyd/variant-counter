# variant-counter

The efficient and elegant crate to count variants of Rust's Enum.

## Get started

### `#[derive(VariantCount)]`

```rust
#[derive(VariantCount)]
pub enum Enum {
    Variant1,
    Variant2,
}
```

### Record your variant

```rust
let mut counter = Enum::counter();
counter.record(&Enum::Variant1);
```

### Erase the record with `erase_*()` methods

```rust
counter.erase_variant1();
```

Those `erase_*()` methods are under `erase` feature flag, and disabled by default.

### Check the record with `check_*()` methods

```rust
assert_eq!(counter.check_variant1(), 1);
```

Those `check_*()` methods are under `check` feature flag, and disabled by default.

### `discard()`, or `reset()` the data

```rust
// Clear the `Enum::Variant1`'s data.
counter.discard(&Enum::Variant1);

// Clear all variants data.
counter.reset();
```

### Ignore a variant

```rust
#[derive(VariantCount)]
pub enum Level {
    #[counter(ignore)]
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}
```

If a variant was ignored, it has no effect when you record that variant.

```rust
let mut counter = Level::counter();
// Record nothing...
counter.record(&Level::Trace);

```
### Aggregate your data

```rust
let data = counter.aggregate();
```

### Group variants

```rust
#[derive(VariantCount)]
pub enum Platform {
    #[counter(group = "Mobile")]
    Android,
    #[counter(group = "Mobile")]
    IOS,
    #[counter(group = "Desktop")]
    Windows,
    #[counter(group = "Desktop")]
    Linux,
    #[counter(group = "Desktop")]
    MacOS,
    #[counter(group = "Desktop")]
    ChromeOS,
    Others,
}

let counter = Platform::counter();
// Group version of aggregate method
let group_data = counter.group_aggregate();
```
### Statistics

```rust
// Sum
counter.sum();

// Average
counter.avg();

// Variance
counter.variance();

// Standard deviation
counter.sd();
```

### Weighted

```rust
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

let mut counter = Rating::counter();
counter.record(&Rating::Loved);

let w = counter.weighted();

// Sum
w.sum();

// Average
w.avg();

// Variance
w.variance();

// Standard deviation
w.sd();
```

## Macro expand

You can use [carg-expand](https://crates.io/crates/cargo-expand) to expand the derived `VariantCount` macro. 
Here is the expanded code:


## Feature flags

- `full`: Enable all features.

- `check`: Generate `check` methods for variants.

- `erase`: Generate `erase` methods for variants.

- `stats`: Generate statistics methods, such as `avg()`, `variance()`, and `sd`, etc.

- `std`: Enable `std` crate supported. Enabled by default. Please disable this feature to support `no_std`.