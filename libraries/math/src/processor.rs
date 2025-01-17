//! Program state processor

use {
    crate::{approximations::sqrt, instruction::MathInstruction, precise_number::PreciseNumber},
    borsh::BorshDeserialize,
    paychains_program::{
        account_info::AccountInfo, entrypoint::ProgramResult, log::pay_log_compute_units, msg,
        pubkey::Pubkey,
    },
};

/// u64_multiply
#[inline(never)]
fn u64_multiply(multiplicand: u64, multiplier: u64) -> u64 {
    multiplicand * multiplier
}

/// u64_divide
#[inline(never)]
fn u64_divide(dividend: u64, divisor: u64) -> u64 {
    dividend / divisor
}

/// f32_multiply
#[inline(never)]
fn f32_multiply(multiplicand: f32, multiplier: f32) -> f32 {
    multiplicand * multiplier
}

/// f32_divide
#[inline(never)]
fn f32_divide(dividend: f32, divisor: f32) -> f32 {
    dividend / divisor
}

/// Instruction processor
pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = MathInstruction::try_from_slice(input).unwrap();
    match instruction {
        MathInstruction::PreciseSquareRoot { radicand } => {
            msg!("Calculating square root using PreciseNumber");
            let radicand = PreciseNumber::new(radicand as u128).unwrap();
            pay_log_compute_units();
            let result = radicand.sqrt().unwrap().to_imprecise().unwrap() as u64;
            pay_log_compute_units();
            msg!("{}", result);
            Ok(())
        }
        MathInstruction::SquareRootU64 { radicand } => {
            msg!("Calculating u64 square root");
            pay_log_compute_units();
            let result = sqrt(radicand).unwrap();
            pay_log_compute_units();
            msg!("{}", result);
            Ok(())
        }
        MathInstruction::SquareRootU128 { radicand } => {
            msg!("Calculating u128 square root");
            pay_log_compute_units();
            let result = sqrt(radicand).unwrap();
            pay_log_compute_units();
            msg!("{}", result);
            Ok(())
        }
        MathInstruction::U64Multiply {
            multiplicand,
            multiplier,
        } => {
            msg!("Calculating U64 Multiply");
            pay_log_compute_units();
            let result = u64_multiply(multiplicand, multiplier);
            pay_log_compute_units();
            msg!("{}", result);
            Ok(())
        }
        MathInstruction::U64Divide { dividend, divisor } => {
            msg!("Calculating U64 Divide");
            pay_log_compute_units();
            let result = u64_divide(dividend, divisor);
            pay_log_compute_units();
            msg!("{}", result);
            Ok(())
        }
        MathInstruction::F32Multiply {
            multiplicand,
            multiplier,
        } => {
            msg!("Calculating f32 Multiply");
            pay_log_compute_units();
            let result = f32_multiply(multiplicand, multiplier);
            pay_log_compute_units();
            msg!("{}", result as u64);
            Ok(())
        }
        MathInstruction::F32Divide { dividend, divisor } => {
            msg!("Calculating f32 Divide");
            pay_log_compute_units();
            let result = f32_divide(dividend, divisor);
            pay_log_compute_units();
            msg!("{}", result as u64);
            Ok(())
        }
        MathInstruction::Noop => {
            msg!("Do nothing");
            msg!("{}", 0_u64);
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::MathInstruction;
    use borsh::BorshSerialize;

    #[test]
    fn test_u64_multiply() {
        assert_eq!(2 * 2, u64_multiply(2, 2));
        assert_eq!(4 * 3, u64_multiply(4, 3));
    }

    #[test]
    fn test_u64_divide() {
        assert_eq!(1, u64_divide(2, 2));
        assert_eq!(2, u64_divide(2, 1));
    }

    #[test]
    fn test_f32_multiply() {
        assert_eq!(2.0 * 2.0, f32_multiply(2.0, 2.0));
        assert_eq!(4.0 * 3.0, f32_multiply(4.0, 3.0));
    }

    #[test]
    fn test_f32_divide() {
        assert_eq!(1.0, f32_divide(2.0, 2.0));
        assert_eq!(2.0, f32_divide(2.0, 1.0));
    }

    #[test]
    fn test_process_instruction() {
        let program_id = Pubkey::new_unique();
        for math_instruction in &[
            MathInstruction::PreciseSquareRoot { radicand: u64::MAX },
            MathInstruction::SquareRootU64 { radicand: u64::MAX },
            MathInstruction::SquareRootU128 {
                radicand: u128::MAX,
            },
            MathInstruction::U64Multiply {
                multiplicand: 3,
                multiplier: 4,
            },
            MathInstruction::U64Divide {
                dividend: 2,
                divisor: 2,
            },
            MathInstruction::F32Multiply {
                multiplicand: 3.0,
                multiplier: 4.0,
            },
            MathInstruction::F32Divide {
                dividend: 2.0,
                divisor: 2.0,
            },
            MathInstruction::Noop,
        ] {
            let input = math_instruction.try_to_vec().unwrap();
            process_instruction(&program_id, &[], &input).unwrap();
        }
    }
}
