#![cfg(feature = "test-bpf")]

use bgl_dialog_manager::{
    instruction::{AppendDialogArgs, CreateDialogArgs},
    pda::find_dialog_pda,
    state::DialogAccount,
};
use borsh::BorshDeserialize;
use solana_program_test::{tokio, ProgramTest};
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};

#[tokio::test]
async fn create_dialog() {
    let mut context = ProgramTest::new("bgl_dialog_manager", bgl_dialog_manager::ID, None)
        .start_with_context()
        .await;

    let npc_mint = Keypair::new();
    let dialog = "You can tell a lot about a loaf by his hat. Which do you choose?".to_string();
    let create_args = CreateDialogArgs {
        npc_mint: npc_mint.pubkey(),
        dialog,
    };

    let dialog_pda = find_dialog_pda(&npc_mint.pubkey(), &context.payer.pubkey());

    let create_dialog_ix = bgl_dialog_manager::instruction::create_dialog(
        &dialog_pda.0,
        &context.payer.pubkey(),
        create_args,
    );

    let append_args_0 = AppendDialogArgs {
        npc_mint: npc_mint.pubkey(),
        prev: 0,
        choice: "Sombrero".to_string(),
        dialog: "Nice choice. A smart loaf who brings the shade with them.".to_string(),
    };

    let append_dialog_ix_0 = bgl_dialog_manager::instruction::append_dialog(
        &dialog_pda.0,
        &context.payer.pubkey(),
        append_args_0,
    );

    let append_args_1 = AppendDialogArgs {
        npc_mint: npc_mint.pubkey(),
        prev: 0,
        choice: "Cowboy hat".to_string(),
        dialog: "An excellent choice for a master of the lasso.".to_string(),
    };

    let append_dialog_ix_1 = bgl_dialog_manager::instruction::append_dialog(
        &dialog_pda.0,
        &context.payer.pubkey(),
        append_args_1,
    );

    let tx = Transaction::new_signed_with_payer(
        &[create_dialog_ix, append_dialog_ix_0, append_dialog_ix_1],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );
    context.banks_client.process_transaction(tx).await.unwrap();

    let dialog_account = context
        .banks_client
        .get_account(dialog_pda.0)
        .await
        .unwrap();

    assert!(dialog_account.is_some());

    let dialog_account = dialog_account.unwrap();

    let mut dialog_account_data = dialog_account.data.as_ref();
    let dialog = DialogAccount::deserialize(&mut dialog_account_data).unwrap();
    // assert_eq!(dialog.data.foo, 1);
    // assert_eq!(dialog.data.bar, 2);
    println!("{:#?}", dialog);
}
