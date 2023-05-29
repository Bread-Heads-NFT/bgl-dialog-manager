pub mod append_dialog;
pub mod create_dialog;
pub mod create_visitor;

pub use append_dialog::*;
pub use create_dialog::*;
pub use create_visitor::*;

use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;

#[derive(Debug, Clone, ShankInstruction, BorshSerialize, BorshDeserialize)]
#[rustfmt::skip]
pub enum DialogManagerInstruction {
    /// Create Dialog for an NPC
    #[account(0, writable, name="dialog_pda", desc = "The address of the new account")]
    #[account(1, writable, signer, name="payer", desc = "The account paying for the storage fees")]
    #[account(2, name="system_program", desc = "The system program")]
    CreateDialog(CreateDialogArgs),

    /// Create Dialog state for a conversation between a player and an NPC
    #[account(0, writable, name="visitor_pda", desc = "The address of the new account")]
    #[account(1, name="dialog_pda", desc = "The address dialog")]
    #[account(2, writable, signer, name="payer", desc = "The account paying for the storage fees")]
    #[account(3, name="system_program", desc = "The system program")]
    CreateVisitor(CreateVisitorArgs),

    /// Append to Dialog for an NPC
    #[account(0, writable, name="dialog_pda", desc = "The address of the new account")]
    #[account(1, writable, signer, name="payer", desc = "The account paying for the storage fees")]
    #[account(2, name="system_program", desc = "The system program")]
    AppendDialog(AppendDialogArgs),
}
