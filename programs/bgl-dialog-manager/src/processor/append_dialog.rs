use crate::error::BglDialogManagerError;
use crate::instruction::AppendDialogArgs;
use crate::state::{DialogAccount, DIALOG};
use mpl_utils::assert_derivation;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    system_program,
};

pub fn append_dialog(accounts: &[AccountInfo], args: AppendDialogArgs) -> ProgramResult {
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

    let mut dialog = DialogAccount::load(dialog_pda_info)?;
    dialog.dialog.append(args.prev, &args.choice, &args.dialog);
    dialog.save(dialog_pda_info, payer_info, system_program)?;

    Ok(())
}
