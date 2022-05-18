//! Use predicates, expressions, and sort descriptors to examine elements in collections and other services.

/// Constants that indicate sort order.
#[derive(Debug, PartialEq, Eq)]
#[repr(C)]
pub enum ComparisonResult {
    /// The left operand is smaller than the right operand.
    OrderedAscending = -1,
    /// The two operands are equal.
    OrderedSame = 0,
    /// The left operand is greater than the right operand.
    OrderedDescending = 1,
}
