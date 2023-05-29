#![cfg(feature = "test-bpf")]

use bgl_dialog_manager::{
    instruction::{CreateDialogArgs, CreateVisitorArgs},
    pda::find_dialog_pda,
    state::VisitorAccount,
};
use borsh::BorshDeserialize;
use solana_program_test::{tokio, ProgramTest};
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};

#[tokio::test]
async fn create_visitor() {
    let mut context = ProgramTest::new("bgl_dialog_manager", bgl_dialog_manager::ID, None)
        .start_with_context()
        .await;

    let npc_mint = Keypair::new();
    let dialog = "Hey, you. You're finally awake.".to_string();
    let create_dialogs_args = CreateDialogArgs {
        npc_mint: npc_mint.pubkey(),
        dialog,
    };

    let dialog_pda = find_dialog_pda(&npc_mint.pubkey(), &context.payer.pubkey());

    let create_dialog_ix = bgl_dialog_manager::instruction::create_dialog(
        &dialog_pda.0,
        &context.payer.pubkey(),
        create_dialogs_args,
    );

    let player_mint = Keypair::new();
    let create_visitor_args = CreateVisitorArgs {
        player_mint: player_mint.pubkey(),
        npc_mint: npc_mint.pubkey(),
    };

    let visitor_pda = bgl_dialog_manager::pda::find_visitor_pda(
        &npc_mint.pubkey(),
        &player_mint.pubkey(),
        &context.payer.pubkey(),
    );

    let create_visitor_ix = bgl_dialog_manager::instruction::create_visitor(
        &visitor_pda.0,
        &dialog_pda.0,
        &context.payer.pubkey(),
        create_visitor_args,
    );

    let tx = Transaction::new_signed_with_payer(
        &[create_dialog_ix, create_visitor_ix],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );
    context.banks_client.process_transaction(tx).await.unwrap();

    let visitor_account = context
        .banks_client
        .get_account(visitor_pda.0)
        .await
        .unwrap();

    assert!(visitor_account.is_some());

    let visitor_account = visitor_account.unwrap();

    let mut visitor_account_data = visitor_account.data.as_ref();
    let visitor: VisitorAccount = VisitorAccount::deserialize(&mut visitor_account_data).unwrap();
    // assert_eq!(dialog.data.foo, 1);
    // assert_eq!(dialog.data.bar, 2);
    println!("{:#?}", visitor);
}
