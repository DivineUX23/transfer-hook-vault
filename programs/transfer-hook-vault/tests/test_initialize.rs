
use {
    anchor_lang::{
        AccountDeserialize, InstructionData, Key, ToAccountMetas, prelude::msg, 
        solana_program::{instruction::Instruction, program_pack::Pack}, system_program::ID as SYSTEM_PROGRAM_ID
    }, 
    anchor_spl::{associated_token::{ID as ASSOCIATED_TOKEN_PROGRAM_ID, spl_associated_token_account::instruction}, token::spl_token, 
        token_2022::{
            ID as TOKEN_2022_PROGRAM_ID, 
            spl_token_2022::{
                extension::StateWithExtensions, instruction::transfer_checked, state::Account as TokenAccount2022
            },
        }, token_interface::Mint
    },
    litesvm::LiteSVM, 
    litesvm_token::{
        CreateAssociatedTokenAccount, spl_token::ID as TOKEN_PROGRAM_ID 
    },
    solana_keypair::Keypair, solana_message::{
        AccountMeta, Address, Message, VersionedMessage
    }, 
    solana_pubkey::Pubkey, 
    solana_signer::Signer, 
    solana_transaction::versioned::VersionedTransaction,
    spl_tlv_account_resolution::state::ExtraAccountMetaList,

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

    let reciever_key = Keypair::new();
    let reciever = reciever_key.pubkey();

    let config = Pubkey::find_program_address(
        &[b"config"],
        &program_id
    ).0;

    let whitelist = Pubkey::find_program_address(
        &[b"Whitelist"],
        &program_id
    ).0;

    let balance = Pubkey::find_program_address(
        &[b"balance", whitelist.key().as_ref()],
        &program_id
    ).0;



    let create_mint = Keypair::new();
    let mint = create_mint.pubkey();

    let extra_account_meta_list = Pubkey::find_program_address(
        &[b"extra-account-meta", mint.key().as_ref()],
        &program_id
    ).0;

    let amount: u64 = 10000;


    //svm.airdrop(&admin.pubkey(), 1_000_000_000).unwrap();
    //svm.airdrop(&reciever.pubkey(), 1_000_000_000).unwrap();


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
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&payer]).unwrap();

    let res = svm.send_transaction(tx).unwrap();

    // Log transaction details
    msg!("\n\n Initialize transaction sucessfull");
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
            amount: amount,
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
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&payer]).unwrap();

    let res = svm.send_transaction(tx).unwrap();

    // Log transaction details
    msg!("\n\n Add Whitelist transaction sucessfull");
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
    assert_eq!(whitelist_data.accounts[0].amount, amount);
    assert_eq!(balance_data.amount, amount);



    // TEST MINT ACCOUNT

    let instruction = Instruction::new_with_bytes(
        program_id,
        &transfer_hook_vault::instruction::InitMint {}.data(),
        transfer_hook_vault::accounts::MintToken {
            user: admin,
            mint: mint,
            config: config,
            token_program: TOKEN_2022_PROGRAM_ID,
            system_program: SYSTEM_PROGRAM_ID

        }.to_account_metas(None),
    );

    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[instruction], Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&create_mint, &payer]).unwrap();

    let res = svm.send_transaction(tx).unwrap();

    // Log transaction details
    msg!("\n\n Mint Token transaction sucessfull");
    msg!("CUs Consumed: {}", res.compute_units_consumed);
    msg!("Tx Signature: {}", res.signature);

    let mint_account = svm.get_account(&mint).unwrap();
    
    let mint_data = Mint::try_deserialize(
        &mut mint_account.data.as_ref(),
    ).unwrap();

    assert_eq!(mint_data.decimals, 6);
    assert_eq!(mint_data.is_initialized, true);
    assert_eq!(mint_data.supply, 0);



    // CREATE ATA's:
    
    let source_ata = CreateAssociatedTokenAccount::new(&mut svm, &payer, &mint)
        .owner(&admin)
        .token_program_id(&TOKEN_2022_PROGRAM_ID)
        .send()
        .unwrap();

    let destination_ata = CreateAssociatedTokenAccount::new(&mut svm, &payer, &mint)
        .owner(&reciever)
        .token_program_id(&TOKEN_2022_PROGRAM_ID)
        .send()
        .unwrap();


    // TEST SEND TOKEN ACCOUNT

    let instruction = Instruction::new_with_bytes(
        program_id,
        &transfer_hook_vault::instruction::InitDistribute {
            amount: amount
        }.data(),
        transfer_hook_vault::accounts::DistributeToken {
            user: admin,
            mint: mint,
            config: config,
            destination_ata: source_ata,
            associated_token_program: ASSOCIATED_TOKEN_PROGRAM_ID,
            token_program: TOKEN_2022_PROGRAM_ID,
            system_program: SYSTEM_PROGRAM_ID

        }.to_account_metas(None),
    );

    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[instruction], Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&payer]).unwrap();

    let res = svm.send_transaction(tx).unwrap();

    // Log transaction details
    msg!("\n\n DistributeToken transaction sucessfull");
    msg!("CUs Consumed: {}", res.compute_units_consumed);
    msg!("Tx Signature: {}", res.signature);

    let mint_account = svm.get_account(&mint).unwrap();
    let mint_data = Mint::try_deserialize(
        &mut mint_account.data.as_ref(),
    ).unwrap();

    let source_account = svm.get_account(&source_ata).unwrap();
    let source_data = StateWithExtensions::<TokenAccount2022>::unpack(
        &source_account.data
    ).unwrap();


    assert_eq!(mint_data.supply, amount);
    assert_eq!(source_data.base.amount, amount);




    
    // TEST Initialize Account Meta List WHITELIST

    let instruction = Instruction::new_with_bytes(
        program_id,
        &transfer_hook_vault::instruction::InitTransferHook {}.data(),
        transfer_hook_vault::accounts::InitializeAccountMetaList {
            admin : admin,
            mint: mint,
            extra_account_meta_list: extra_account_meta_list,
            system_program: SYSTEM_PROGRAM_ID

        }.to_account_metas(None),
    );

    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[instruction], Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&payer]).unwrap();

    let res = svm.send_transaction(tx).unwrap();

    // Log transaction details
    msg!("\n\n Initialize Account Meta transaction sucessfull");
    msg!("CUs Consumed: {}", res.compute_units_consumed);
    msg!("Tx Signature: {}", res.signature);

    let extra_account = svm.get_account(&extra_account_meta_list).unwrap();
    let extra_data = &extra_account.data;

    let mut extra_length = [0u8; 4];
    extra_length.copy_from_slice(&extra_data[8..12]);

    let extra_data_length = u32::from_le_bytes(extra_length);

    let count_data = extra_data_length / 35;
    
    //print!("THIS IS IT'S SIZE {}", count_data);
    assert_eq!(count_data, 2);




    // TEST TRANSFER HOOK

    let instruction = Instruction::new_with_bytes(
        program_id,
        &transfer_hook_vault::instruction::Execute {
            amount: amount,
        }.data(),
        transfer_hook_vault::accounts::TransferHook {
            sender: admin,
            mint: mint,
            source_ata: source_ata,
            destination_ata: destination_ata,
            whitelist: whitelist,
            balance: balance,
        }.to_account_metas(None),
    );

    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[instruction], Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&payer]).unwrap();

    let res = svm.send_transaction(tx);

    // Log transaction details
    msg!("\n\n Add TRANSFER HOOK transaction sucessfull");
    msg!("CUs Consumed: {}", res.is_err());

    assert!(res.is_err(), "Not Transferring");



    // EXPERIMNET: TEST SEND
    /* 

    // TEST SEND PASS
    let amount_send: u64 = 8000;

    
    let mut instruction = transfer_checked(
        &TOKEN_2022_PROGRAM_ID, 
        &source_ata.key(), 
        &mint.key(), 
        &destination_ata.key(), 
        &admin.key(), 
        &[&admin.key(), &reciever.key()], 
        amount_send,
        6
    ).unwrap();

    instruction.accounts.push(AccountMeta::new_readonly(extra_account_meta_list, false));
    instruction.accounts.push(AccountMeta::new_readonly(whitelist, false));
    instruction.accounts.push(AccountMeta::new_readonly(balance, false));
    instruction.accounts.push(AccountMeta::new_readonly(program_id, false));


    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[instruction], Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&reciever_key, &payer]).unwrap();


    let res = svm.send_transaction(tx).unwrap();
    // Log transaction details
    msg!("\n\n SEND PASS transaction sucessfull");
    msg!("CUs Consumed: {}", res.compute_units_consumed);
    msg!("Tx Signature: {}", res.signature);


    let source_account = svm.get_account(&source_ata).unwrap();
    let source_data = StateWithExtensions::<TokenAccount2022>::unpack(
        &source_account.data
    ).unwrap();

    let destination_account = svm.get_account(&destination_ata).unwrap();
    let destination_data = StateWithExtensions::<TokenAccount2022>::unpack(
        &destination_account.data
    ).unwrap();

    assert_eq!(source_data.base.amount, 2000);
    assert_eq!(destination_data.base.amount, amount_send);
    */

    /* 
    let res = svm.send_transaction(tx).unwrap();
    let instruction = Instruction::new_with_bytes(
        program_id,
        &transfer_hook_vault::instruction::SendToken {
            amount: amount_send,
        }.data(),
        transfer_hook_vault::accounts::SendToken {
            admin: admin,
            sender: reciever,
            mint: mint,
            source_ata: source_ata,
            destination_ata: destination_ata,
            program_id: program_id,
            whitelist: whitelist,
            balance: balance,
            extra_account_meta_list: extra_account_meta_list,
            token_program: TOKEN_2022_PROGRAM_ID,
        }.to_account_metas(None),
    );

    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[instruction], Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&reciever_key, &payer]).unwrap();
    */


    /* 
    // TEST SEND FAIL

    let amount_send: u64 = 100;

    let instruction = Instruction::new_with_bytes(
        program_id,
        &transfer_hook_vault::instruction::SendToken {
            amount: amount_send,
        }.data(),
        transfer_hook_vault::accounts::SendToken {
            admin: admin,
            sender: reciever,
            mint: mint,
            source_ata: source_ata,
            destination_ata: destination_ata,
            program_id: program_id,
            whitelist: whitelist,
            balance: balance,
            extra_account_meta_list: extra_account_meta_list,
            token_program: TOKEN_2022_PROGRAM_ID,
        }.to_account_metas(None),
    );

    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[instruction], Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&reciever_key, &payer]).unwrap();

    let res = svm.send_transaction(tx);
    
    // Log transaction details
    msg!("\n\n Add SEND FAIL 2 transaction sucessfull");

    assert!(res.is_err(), "Not Transferring");

    let resun = res.unwrap();

    msg!("CUs Consumed: {}", resun.compute_units_consumed);
    msg!("Tx Signature: {}", resun.signature);


    let source_account = svm.get_account(&source_ata).unwrap();
    let source_data = StateWithExtensions::<TokenAccount2022>::unpack(
        &source_account.data
    ).unwrap();

    let destination_account = svm.get_account(&destination_ata).unwrap();
    let destination_data = StateWithExtensions::<TokenAccount2022>::unpack(
        &destination_account.data
    ).unwrap();

    assert_eq!(source_data.base.amount, 1900);
    assert_eq!(destination_data.base.amount, 8100);
    */



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
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&payer]).unwrap();

    let res = svm.send_transaction(tx).unwrap();

    // Log transaction details
    msg!("\n\n Remove Whitelist transaction sucessfull");
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
