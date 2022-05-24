use bincode::{
    de::Decoder,
    enc::Encoder,
    error::{DecodeError, EncodeError},
    Decode, Encode,
};
use std::fmt;

/// Represents a typed collection of values.
///
/// Programs take different types of values as input. What matters is the order
/// of values inside each type. The order between types doesn't matter. The
/// kind of initial secret sharing also affects the type of values.
#[derive(Clone, Debug, PartialEq)]
pub struct Values {
    /// The values for B8.
    pub b8: Vec<u8>,
}

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

impl Encode for Operation {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        use Operation::*;

        let opcode: u8 = match self {
            Xor => 0x40,
            And => 0x41,
        };
        opcode.encode(encoder)
    }
}

impl Decode for Operation {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        use Operation::*;

        let opcode = u8::decode(decoder)?;
        let op = match opcode {
            0x40 => Xor,
            0x41 => And,
            _ => {
                return Err(DecodeError::OtherString(format!(
                    "invalid opcode: {}",
                    opcode
                )))
            }
        };
        Ok(op)
    }
}

/// This describes what inputs a function takes, and what outputs it produces.
#[derive(Clone, Copy, Encode, Decode, Debug, PartialEq)]
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
#[derive(Clone, Encode, Decode, Debug, PartialEq)]
pub struct Function {
    /// The signature describing the inputs and the outputs of this function.
    pub signature: FunctionSignature,
    /// The operations making up the function.
    ///
    /// The function implicitly returns after reaching the last operation.
    pub operations: Vec<Operation>,
}

/// Represents a program in our bytecode.
///
/// This is what gets executed, and what we create proofs for.
///
/// At the moment, a program consists of a single function. This function
/// defines the input and output of the program.
#[derive(Clone, Encode, Decode, Debug, PartialEq)]
pub struct Program {
    /// The main function, and entry point of the program.
    pub main: Function,
}

#[cfg(test)]
mod test {
    use super::*;
    use bincode::{config, decode_from_slice, encode_to_vec};

    fn test_encode_roundtrip(program: &Program) {
        let config = config::standard();
        let encoded = encode_to_vec(program, config);
        assert!(encoded.is_ok());
        let encoded = encoded.unwrap();
        let decoded = decode_from_slice(&encoded, config);
        assert!(decoded.is_ok());
        let (decoded, _): (Program, usize) = decoded.unwrap();
        assert_eq!(*program, decoded);
    }

    #[test]
    fn test_encode_examples() {
        test_encode_roundtrip(&Program {
            main: Function {
                signature: FunctionSignature {
                    inputs: 3,
                    outputs: 1,
                },
                operations: vec![Operation::Xor, Operation::And],
            },
        });
    }
}
