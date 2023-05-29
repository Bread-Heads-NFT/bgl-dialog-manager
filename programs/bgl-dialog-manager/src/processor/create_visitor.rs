use crate::error::BglDialogManagerError;
use crate::instruction::CreateVisitorArgs;
use crate::state::{DialogAccount, Key, VisitorAccount, DIALOG, VISITOR};
use borsh::BorshSerialize;
use mpl_utils::{assert_derivation, create_or_allocate_account_raw};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    system_program,
};

pub fn create_visitor(accounts: &[AccountInfo], args: CreateVisitorArgs) -> ProgramResult {
    // Accounts.
    let account_info_iter = &mut accounts.iter();
    let visitor_pda_info = next_account_info(account_info_iter)?;
    let dialog_pda_info = next_account_info(account_info_iter)?;
    let payer_info = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    // Guards.
    if *system_program.key != system_program::id() {
        return Err(BglDialogManagerError::InvalidSystemProgram.into());
    }

    let payer_address = payer_info.key.to_bytes();
    let npc_mint = args.npc_mint.to_bytes();
    let dialog_seeds = vec![DIALOG.as_bytes(), &npc_mint, &payer_address];

    let bump = &[assert_derivation(
        &crate::ID,
        dialog_pda_info,
        &dialog_seeds,
        BglDialogManagerError::DerivedKeyInvalid,
    )?];

    let dialog = DialogAccount::load(dialog_pda_info)?;

    if dialog.bump != bump[0] {
        return Err(BglDialogManagerError::DerivedKeyInvalid.into());
    }

    let player_mint = args.player_mint.to_bytes();
    let mut visitor_seeds = vec![VISITOR.as_bytes(), &npc_mint, &player_mint, &payer_address];

    let bump = &[assert_derivation(
        &crate::ID,
        visitor_pda_info,
        &visitor_seeds,
        BglDialogManagerError::DerivedKeyInvalid,
    )?];

    visitor_seeds.push(bump);

    let visitor_account = VisitorAccount {
        key: Key::VisitorAccount,
        bump: bump[0],
        index: 0,
        path: vec![],
    };

    let bytes = visitor_account.try_to_vec()?;

    create_or_allocate_account_raw(
        crate::ID,
        visitor_pda_info,
        system_program,
        payer_info,
        bytes.len(),
        &visitor_seeds,
    )?;

    visitor_pda_info.try_borrow_mut_data().unwrap()[..bytes.len()].copy_from_slice(&bytes);

    Ok(())
}
