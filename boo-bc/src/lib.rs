use std::fmt;

/// Represents a single operation in our bytecode.
///
/// Each of the operations manipulates values on the stack, potentially performing
/// some kind of bitwise operation or other arithmetic on those values.
#[derive(Clone, Debug, PartialEq)]
enum Operation {
    /// Xor the top two elements of the stack, replacing them.
    Xor,
    /// And the top two elements of the stack, replacing them.
    And,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Operation::*;

        match self {
            Xor => write!(f, "Xor"),
            And => write!(f, "And"),
        }
    }
}
