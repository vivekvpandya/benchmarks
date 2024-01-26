use p3_field::ExtensionField;
use p3_field::PrimeField64;
use valida_assembler::assemble;
use valida_machine::{InstructionWord, ProgramROM};

pub fn generate_sha256_program<Val: PrimeField64, Challenge: ExtensionField<Val>>(
) -> Vec<InstructionWord<i32>> {
    let assembly_code = include_str!("sha256.val");
    let machine_code = assemble(assembly_code).unwrap();
    let program = ProgramROM::from_machine_code(&machine_code);
    program.0
}
