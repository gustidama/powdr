/// Module providing common values and utilities for steps from asm to PIL.
use number::FieldElement;
use crate::utils::parse_instruction;

/// Constant for the `return` keyword in the PIL constraints.
pub const RETURN_NAME: &str = "return";
/// Constant for the `reset` instruction in the PIL constraints.
pub const RESET_NAME: &str = "_reset";

/// Generates the instruction flag name for a given instruction.
///
/// # Arguments
///
/// * `name` - A string slice that holds the name of the instruction.
///
/// # Returns
///
/// A `String` that represents the instruction flag.
pub fn instruction_flag(name: &str) -> String {
    format!("instr_{name}")
}

/// Generates the names of the output assignment registers for a given count.
///
/// # Arguments
///
/// * `count` - The number of output registers.
///
/// # Returns
///
/// A `Vec<String>` containing the names of the output assignment registers.
fn output_registers(count: usize) -> Vec<String> {
    (0..count).map(output_at).collect()
}

/// Generates the name of the read-only register at a given index.
///
/// # Arguments
///
/// * `i` - The index of the register.
///
/// # Returns
///
/// A `String` that represents the name of the read-only register.
pub fn input_at(i: usize) -> String {
    format!("_input_{}", i)
}

/// Generates the name of the output assignment register at a given index.
///
/// # Arguments
///
/// * `i` - The index of the register.
///
/// # Returns
///
/// A `String` that represents the name of the output assignment register.
pub fn output_at(i: usize) -> String {
    format!("_output_{}", i)
}

/// Generates the return instruction for a given number of outputs and program counter name.
///
/// # Arguments
///
/// * `output_count` - The number of output registers.
/// * `pc_name` - The name of the program counter.
///
/// # Returns
///
/// An `Instruction` instance representing the return instruction.
pub fn return_instruction<T: FieldElement>(
    output_count: usize,
    pc_name: &str,
) -> ast::asm_analysis::Instruction<T> {
    parse_instruction(&format!(
        "{} {{ {pc_name}' = 0 }}",
        output_registers(output_count).join(", ")
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_flag() {
        assert_eq!(instruction_flag("test"), "instr_test");
    }

    #[test]
    fn test_output_registers() {
        let expected = vec!["_output_0".to_string(), "_output_1".to_string()];
        assert_eq!(output_registers(2), expected);
    }

    #[test]
    fn test_input_at() {
        assert_eq!(input_at(0), "_input_0");
        assert_eq!(input_at(1), "_input_1");
    }

    #[test]
    fn test_output_at() {
        assert_eq!(output_at(0), "_output_0");
        assert_eq!(output_at(1), "_output_1");
    }

    #[test]
    fn test_return_instruction() {
        // This test is hypothetical and assumes the existence of the parse_instruction function and Instruction type.
        let result = return_instruction::<u32>(2, "pc");
        // Validate result with expected value (depends on implementation details of parse_instruction and Instruction).
    }
}
