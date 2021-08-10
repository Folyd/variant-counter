pub use variant_counter_derived::*;

pub trait VariantCount {
    type Target;
    fn counter(&self) -> Self::Target;
}
