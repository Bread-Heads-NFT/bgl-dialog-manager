use crate::error::BglDialogManagerError;
use crate::instruction::CreateDialogArgs;
use crate::state::{DialogAccount, Key, DIALOG};
use borsh::BorshSerialize;
use mpl_utils::{assert_derivation, create_or_allocate_account_raw};
use solana_data_structs::FlatGraph;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    system_program,
};

pub fn create_dialog(accounts: &[AccountInfo], args: CreateDialogArgs) -> ProgramResult {
    // Accounts.
    let account_info_iter = &mut accounts.iter();
    let dialog_pda_info = next_account_info(account_info_iter)?;
    let payer_info = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    // Guards.
    if *system_program.key != system_program::id() {
        return Err(BglDialogManagerError::InvalidSystemProgram.into());
    }

    let payer_address = payer_info.key.to_bytes();
    let npc_mint = args.npc_mint.to_bytes();
    let mut dialog_seeds = vec![DIALOG.as_bytes(), &npc_mint, &payer_address];

    let bump = &[assert_derivation(
        &crate::ID,
        dialog_pda_info,
        &dialog_seeds,
        BglDialogManagerError::DerivedKeyInvalid,
    )?];

    dialog_seeds.push(bump);

    let dialog_account = DialogAccount {
        key: Key::DialogAccount,
        bump: bump[0],
        dialog: FlatGraph::new(&args.dialog),
    };

    let bytes = dialog_account.try_to_vec()?;

    create_or_allocate_account_raw(
        crate::ID,
        dialog_pda_info,
        system_program,
        payer_info,
        bytes.len(),
        &dialog_seeds,
    )?;

    dialog_pda_info.try_borrow_mut_data().unwrap()[..bytes.len()].copy_from_slice(&bytes);

    Ok(())
}
