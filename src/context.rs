#![crate_type = "lib"]
#![crate_name = "big"]

pub enum Accuracy {
    Below = -1,
    Exact =  0,
    Above =  1,
}

/// Context tells the arithmetic operations how to do their jobs.
///
/// Prec is the maximum number of digits that should trail
/// the radix during a potentially lossy (e.g., division) operation.
/// The Decimal's precision will only be less than Prec if
/// the operation has a finite expansion less than Prec.
///
/// Mode instructs lossy operations how to round.
pub struct Context {
    prec: i32,
    mode: RoundingMode,
}

/// RoundingMode determines how a Decimal will be rounded
/// if the exact result cannot be accurately represented.
pub enum RoundingMode {
    ToNearestEven,
    ToNearestAway,
    ToZero,
    AwayFromZero,
    ToNegativeInf,
    ToPositiveInf,
    Unneeded,
}

pub const DEFAULT_PREC: i32 = 16;
pub const MAX_SCALE: i32 = i32::MAX;
pub const MIN_SCALE: i32 = i32:MIN + 1;
pub const MIN_PREC: i32 = 0;
pub const MAX_PREC: i32 = i32::MAX;