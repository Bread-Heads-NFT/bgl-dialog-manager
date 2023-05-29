use borsh::{BorshDeserialize, BorshSerialize};
use mpl_utils::resize_or_reallocate_account_raw;
use shank::ShankAccount;
use solana_data_structs::FlatGraph;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program_error::ProgramError;

use crate::error::BglDialogManagerError;

pub const DIALOG: &str = "dialog";
pub const VISITOR: &str = "visitor";

type DialogGraph = FlatGraph<String, String>;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub enum Key {
    Uninitialized,
    DialogAccount,
    VisitorAccount,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, ShankAccount)]
pub struct DialogAccount {
    pub key: Key,
    pub bump: u8,
    pub dialog: DialogGraph,
}

impl DialogAccount {
    pub fn load(account: &AccountInfo) -> Result<Self, ProgramError> {
        let mut bytes: &[u8] = &(*account.data).borrow();
        DialogAccount::deserialize(&mut bytes).map_err(|error| {
            msg!("Error: {}", error);
            BglDialogManagerError::DeserializationError.into()
        })
    }

    pub fn save<'a>(
        &self,
        account: &AccountInfo<'a>,
        payer: &AccountInfo<'a>,
        system_program: &AccountInfo<'a>,
    ) -> ProgramResult {
        let mut bytes = Vec::with_capacity(account.data_len());
        self.serialize(&mut bytes).map_err(|error| {
            msg!("Error: {}", error);
            BglDialogManagerError::SerializationError
        })?;

        resize_or_reallocate_account_raw(account, payer, system_program, bytes.len())?;

        account.try_borrow_mut_data().unwrap()[..bytes.len()].copy_from_slice(&bytes);
        Ok(())
    }
}

#[derive(Debug, BorshDeserialize, BorshSerialize, ShankAccount)]
pub struct VisitorAccount {
    pub key: Key,
    pub bump: u8,
    pub index: usize,
    pub path: Vec<usize>,
}

impl VisitorAccount {
    pub fn load(account: &AccountInfo) -> Result<Self, ProgramError> {
        let mut bytes: &[u8] = &(*account.data).borrow();
        VisitorAccount::deserialize(&mut bytes).map_err(|error| {
            msg!("Error: {}", error);
            BglDialogManagerError::DeserializationError.into()
        })
    }

    pub fn save<'a>(
        &self,
        account: &AccountInfo<'a>,
        payer: &AccountInfo<'a>,
        system_program: &AccountInfo<'a>,
    ) -> ProgramResult {
        let mut bytes = Vec::with_capacity(account.data_len());
        self.serialize(&mut bytes).map_err(|error| {
            msg!("Error: {}", error);
            BglDialogManagerError::SerializationError
        })?;

        resize_or_reallocate_account_raw(account, payer, system_program, bytes.len())?;

        account.try_borrow_mut_data().unwrap()[..bytes.len()].copy_from_slice(&bytes);
        Ok(())
    }
}
