use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use super::DialogManagerInstruction;

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct AppendDialogArgs {
    /// The mint for the NPC NFT that is the source of the dialog.
    pub npc_mint: Pubkey,
    /// The index of the dialog node to append to.
    pub prev: usize,
    /// The choice dialog that leads to the new dialog.
    pub choice: String,
    /// The string for the new dialog node.
    pub dialog: String,
}

pub fn append_dialog(dialog_pda: &Pubkey, payer: &Pubkey, args: AppendDialogArgs) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*dialog_pda, false),
        AccountMeta::new(*payer, true),
        AccountMeta::new_readonly(solana_program::system_program::ID, false),
    ];
    Instruction {
        program_id: crate::ID,
        accounts,
        data: DialogManagerInstruction::AppendDialog(args)
            .try_to_vec()
            .unwrap(),
    }
}
