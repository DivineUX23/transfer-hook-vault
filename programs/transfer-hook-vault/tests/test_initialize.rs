
use {
    anchor_lang::{
        AccountDeserialize, InstructionData, Key, ToAccountMetas, prelude::msg, solana_program::instruction::Instruction, system_program::ID as SYSTEM_PROGRAM_ID
    }, litesvm::LiteSVM, solana_keypair::Keypair, solana_message::{
        Address, Message, VersionedMessage
    }, solana_pubkey::Pubkey, solana_signer::Signer, solana_transaction::versioned::VersionedTransaction
};


fn setup() -> (LiteSVM, Keypair, Address) {
    let program_id = transfer_hook_vault::id();
    let payer = Keypair::new();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/transfer_hook_vault.so");
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

    (svm, payer, program_id)
}

#[test]
fn test_initialize() {

    let (mut svm, payer, program_id) = setup();

    let admin = payer.pubkey();

    let config = Pubkey::find_program_address(
        &[b"config"],
        &program_id
    ).0;

    let whitelist = Pubkey::find_program_address(
        &[b"Whitelist"],
        &program_id
    ).0;

    let balance = Pubkey::find_program_address(
        &[b"Whitelist", whitelist.as_ref()],
        &program_id
    ).0;


    // TEST INITAILIZE

    let instruction = Instruction::new_with_bytes(
        program_id,
        &transfer_hook_vault::instruction::InitializeWhitelist {}.data(),
        transfer_hook_vault::accounts::InitializeWhitelist {
            admin : admin,
            config : config,
            whitelist: whitelist,
            balance: balance,
            system_program: SYSTEM_PROGRAM_ID

        }.to_account_metas(None),
    );

    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[instruction], Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[payer]).unwrap();

    let res = svm.send_transaction(tx).unwrap();

    // Log transaction details
    msg!("\n\nMake transaction sucessfull");
    msg!("CUs Consumed: {}", res.compute_units_consumed);
    msg!("Tx Signature: {}", res.signature);

    let config_account = svm.get_account(&config).unwrap();

    let config_data = transfer_hook_vault::state::Config::try_deserialize(
        &mut config_account.data.as_ref(),
    ).unwrap();

    let whitelist_account = svm.get_account(&whitelist).unwrap();
    
    let whitelist_data = transfer_hook_vault::state::Whitelist::try_deserialize(
        &mut whitelist_account.data.as_ref(),
    ).unwrap();
    
    let balance_account = svm.get_account(&balance).unwrap();

    let balance_data = transfer_hook_vault::state::Balance::try_deserialize(
        &mut balance_account.data.as_ref(),
    ).unwrap();

    assert_eq!(config_data.admin, admin.key());
    assert!(whitelist_data.accounts.is_empty());
    assert_eq!(balance_data.amount, 0);




    // TEST ADD WHITELIST

    let instruction = Instruction::new_with_bytes(
        program_id,
        &transfer_hook_vault::instruction::AddWhitelist {
            user: admin,
            amount: 10000,
        }.data(),
        transfer_hook_vault::accounts::OpsWhitelist {
            admin : admin,
            config : config,
            whitelist: whitelist,
            balance: balance,
            system_program: SYSTEM_PROGRAM_ID

        }.to_account_metas(None),
    );

    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[instruction], Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[payer]).unwrap();

    let res = svm.send_transaction(tx).unwrap();

    // Log transaction details
    msg!("\n\nMake transaction sucessfull");
    msg!("CUs Consumed: {}", res.compute_units_consumed);
    msg!("Tx Signature: {}", res.signature);

    let whitelist_account = svm.get_account(&whitelist).unwrap();
    
    let whitelist_data = transfer_hook_vault::state::Whitelist::try_deserialize(
        &mut whitelist_account.data.as_ref(),
    ).unwrap();
    
    let balance_account = svm.get_account(&balance).unwrap();

    let balance_data = transfer_hook_vault::state::Balance::try_deserialize(
        &mut balance_account.data.as_ref(),
    ).unwrap();

    assert_eq!(whitelist_data.accounts.len(), 1);
    assert_eq!(whitelist_data.accounts[0].user, admin);
    assert_eq!(whitelist_data.accounts[0].amount, 10000);

    assert_eq!(balance_data.amount, 10000);




    // TEST REMOVE WHITELIST

    let instruction = Instruction::new_with_bytes(
        program_id,
        &transfer_hook_vault::instruction::RemoveWhitelist {
            user: admin
        }.data(),
        transfer_hook_vault::accounts::OpsWhitelist {
            admin : admin,
            config : config,
            whitelist: whitelist,
            balance: balance,
            system_program: SYSTEM_PROGRAM_ID

        }.to_account_metas(None),
    );

    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[instruction], Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[payer]).unwrap();

    let res = svm.send_transaction(tx).unwrap();

    // Log transaction details
    msg!("\n\nMake transaction sucessfull");
    msg!("CUs Consumed: {}", res.compute_units_consumed);
    msg!("Tx Signature: {}", res.signature);

    let whitelist_account = svm.get_account(&whitelist).unwrap();
    
    let whitelist_data = transfer_hook_vault::state::Whitelist::try_deserialize(
        &mut whitelist_account.data.as_ref(),
    ).unwrap();
    
    let balance_account = svm.get_account(&balance).unwrap();

    let balance_data = transfer_hook_vault::state::Balance::try_deserialize(
        &mut balance_account.data.as_ref(),
    ).unwrap();

    assert_eq!(whitelist_data.accounts.len(), 0);

    assert_eq!(balance_data.amount, 0);


}
