use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use super::DialogManagerInstruction;

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct CreateDialogArgs {
    /// The mint for the NPC NFT that is the source of the dialog.
    pub npc_mint: Pubkey,
    /// The string for the root dialog node.
    pub dialog: String,
}

pub fn create_dialog(dialog_pda: &Pubkey, payer: &Pubkey, args: CreateDialogArgs) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*dialog_pda, false),
        AccountMeta::new(*payer, true),
        AccountMeta::new_readonly(solana_program::system_program::ID, false),
    ];
    Instruction {
        program_id: crate::ID,
        accounts,
        data: DialogManagerInstruction::CreateDialog(args)
            .try_to_vec()
            .unwrap(),
    }
}
