use std::fmt;

/// Represents a single operation in our bytecode.
///
/// Each of the operations manipulates values on the stack, potentially performing
/// some kind of bitwise operation or other arithmetic on those values.
#[derive(Clone, Debug, PartialEq)]
pub enum Operation {
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

/// This describes what inputs a function takes, and what outputs it produces.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FunctionSignature {
    /// The number of inputs to the function.
    pub inputs: u32,
    /// The number of outputs to the function.
    pub outputs: u32,
}

/// Represents a single function in the bytecode.
///
/// The function takes inputs on the stack, and produces a certain number
/// of outputs.
#[derive(Clone, Debug)]
pub struct Function {
    /// The signature describing the inputs and the outputs of this function.
    pub signature: FunctionSignature,
    /// The operations making up the function.
    ///
    /// The function implicitly returns after reaching the last operation.
    pub operations: Vec<Operation>,
}
