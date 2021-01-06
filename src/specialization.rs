use core::fmt::Debug;

/// Specialized trait used to distinguish between types that implement Debug and one's that don't.
/// ```
/// # use partialdebug::specialization::AsDebug;
/// # struct DNA;
/// # let dna = DNA;
/// assert!(42.as_debug().is_some());
/// assert!(dna.as_debug().is_none());
/// ```
pub trait AsDebug {
    /// Try to get a reference to `self` as `dyn Debug`
    fn as_debug(&self) -> Option<&dyn Debug>;
}

impl<T> AsDebug for T {
    default fn as_debug(&self) -> Option<&dyn Debug> {
        None
    }
}

impl<T: Debug> AsDebug for T {
    fn as_debug(&self) -> Option<&dyn Debug> {
        Some(self)
    }
}
