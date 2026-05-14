
use {
    anchor_lang::{
        InstructionData, ToAccountMetas, 
        solana_program::instruction::Instruction,
        system_program::ID as SYSTEM_PROGRAM_ID
    }, 
    litesvm::LiteSVM, solana_keypair::Keypair, 
    solana_message::{
        Address, Message, VersionedMessage
    }, 
    solana_signer::Signer, 
    solana_transaction::versioned::VersionedTransaction,
    solana_pubkey::Pubkey,
};

#[test]
fn test_initialize() {
    let program_id = transfer_hook_vault::id();
    let payer = Keypair::new();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/transfer_hook_vault.so");
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();
    
    let admin = payer.pubkey();

    let config = Pubkey::find_program_address(
        &[b"config"],
        &program_id
    ).0;

    let whitelist = Pubkey::find_program_address(
        &[b"Whitelist"],
        &program_id
    ).0;

    let instruction = Instruction::new_with_bytes(
        program_id,
        &transfer_hook_vault::instruction::InitializeWhitelist {}.data(),
        transfer_hook_vault::accounts::InitializeWhitelist {
            admin : admin,
            config : config,
            whitelist: whitelist,
            system_program: SYSTEM_PROGRAM_ID

        }.to_account_metas(None),
    );

    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[instruction], Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[payer]).unwrap();

    let res = svm.send_transaction(tx);
    assert!(res.is_ok());
}
