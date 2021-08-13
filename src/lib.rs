pub use variant_counter_derived::*;

pub trait VariantCount {
    type Target;
    fn counter() -> Self::Target;
}
