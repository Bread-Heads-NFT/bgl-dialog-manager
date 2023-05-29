use solana_program::pubkey::Pubkey;

use crate::state::{DIALOG, VISITOR};

pub fn find_dialog_pda(npc_mint: &Pubkey, authority: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[DIALOG.as_bytes(), npc_mint.as_ref(), authority.as_ref()],
        &crate::ID,
    )
}

pub fn find_visitor_pda(
    npc_mint: &Pubkey,
    player_mint: &Pubkey,
    authority: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            VISITOR.as_bytes(),
            npc_mint.as_ref(),
            player_mint.as_ref(),
            authority.as_ref(),
        ],
        &crate::ID,
    )
}
