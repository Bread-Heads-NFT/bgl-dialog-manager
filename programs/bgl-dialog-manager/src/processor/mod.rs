use crate::instruction::DialogManagerInstruction;
use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

pub mod append_dialog;
pub mod create_dialog;
pub mod create_visitor;

use append_dialog::*;
use create_dialog::*;
use create_visitor::*;

pub struct Processor;
impl Processor {
    pub fn process_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction: DialogManagerInstruction =
            DialogManagerInstruction::try_from_slice(instruction_data)?;
        match instruction {
            DialogManagerInstruction::CreateDialog(args) => {
                msg!("Instruction: Create Dialog");
                create_dialog(accounts, args)
            }
            DialogManagerInstruction::CreateVisitor(args) => {
                msg!("Instruction: Create Visitor");
                create_visitor(accounts, args)
            }
            DialogManagerInstruction::AppendDialog(args) => {
                msg!("Instruction: Append Dialog");
                append_dialog(accounts, args)
            }
        }
    }
}
