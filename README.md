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

```rust
enum Enum {
    Variant1,
    Variant2,
}
impl Enum {
    #[inline]
    const fn variant_count() -> usize {
        2usize
    }
}
impl variant_counter::VariantCount for Enum {
    type Counter = EnumCounter;
    fn counter() -> Self::Counter {
        EnumCounter::new()
    }
}
/// The concrete counter struct auto-generated by macro.
#[must_use]
struct EnumCounter {
    /// An array store the frequency of each variant which not be ignored.
    frequency: [usize; 2usize],
}
impl EnumCounter {
    const fn new() -> EnumCounter {
        EnumCounter {
            frequency: [0; 2usize],
        }
    }
    /// Record a variant. It has no effect if you record an ignored variant.
    fn record(&mut self, target: &Enum) {
        let pair = match target {
            Enum::Variant1 => Some(0usize),
            Enum::Variant2 => Some(1usize),
            _ => None,
        };
        if let Some(index) = pair {
            self.frequency[index] = self.frequency[index].saturating_add(1);
        }
    }
    /// Discard the record of the target variant.
    /// It has no effect if you discard an ignored variant.
    fn discard(&mut self, target: &Enum) {
        let index = match target {
            Enum::Variant1 => Some(0usize),
            Enum::Variant2 => Some(1usize),
            _ => None,
        };
        if let Some(index) = index {
            self.frequency[index] = 0;
        }
    }
    /// Reset the records.
    fn reset(&mut self) {
        self.frequency = [0; 2usize];
    }
    /// Aggregate the data to a HashMap.
    #[cfg(feature = "std")]
    fn aggregate(&self) -> std::collections::HashMap<&'static str, usize> {
        IntoIterator::into_iter([
            ("Variant1", self.frequency[0usize]),
            ("Variant2", self.frequency[1usize]),
        ])
        .collect()
    }
    /// Get the sum of frequency.
    #[inline]
    fn sum(&self) -> usize {
        self.frequency.iter().sum()
    }
}
```

## Feature flags

- `full`: Enable all features.

- `check`: Generate `check` methods for variants.

- `erase`: Generate `erase` methods for variants.

- `stats`: Generate statistics methods, such as `avg()`, `variance()`, and `sd`, etc.

- `std`: Enable `std` crate supported. Enabled by default. Please disable this feature to support `no_std`.