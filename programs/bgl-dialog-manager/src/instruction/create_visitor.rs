use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use super::DialogManagerInstruction;

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct CreateVisitorArgs {
    /// The mint for the player NFT that is talking to the NPC.
    pub player_mint: Pubkey,
    /// The mint for the NPC NFT that is the source of the dialog.
    pub npc_mint: Pubkey,
}

pub fn create_visitor(
    visitor_pda: &Pubkey,
    dialog_pda: &Pubkey,
    payer: &Pubkey,
    args: CreateVisitorArgs,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*visitor_pda, false),
        AccountMeta::new_readonly(*dialog_pda, false),
        AccountMeta::new(*payer, true),
        AccountMeta::new_readonly(solana_program::system_program::ID, false),
    ];
    Instruction {
        program_id: crate::ID,
        accounts,
        data: DialogManagerInstruction::CreateVisitor(args)
            .try_to_vec()
            .unwrap(),
    }
}
