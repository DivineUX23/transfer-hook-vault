use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::spl_token_2022::{
        extension::{
            transfer_hook::TransferHookAccount, 
            BaseStateWithExtensionsMut, 
            PodStateWithExtensionsMut
        },
        pod::PodAccount,
    },
    token_interface::{Mint, TokenAccount},
};

use crate::WHITELIST_SEED;

#[derive(Accounts)]
pub struct TransferHook<'info> {
    /// CHECK: source account checked
    pub sender: UncheckedAccount<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        token::mint = mint,
        token::authority = sender
    )]
    pub source_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        token::mint = mint
    )]
    pub destination_atas: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: check done in impl below
    #[account(
        seeds = [WHITELIST_SEED],
        bump
    )]
    pub whitelist: UncheckedAccount<'info>,

}

impl <'info>TransferHook<'info> {

    pub fn transfer_hook(&mut self, amount: u64) -> Result<()> {

        self.is_transferring()?;

        let mut user_exists = false;

        let raw_whitelist = self.whitelist.try_borrow_data()?;

        let mut length_bytes = [0u8; 4];

        length_bytes.copy_from_slice(&raw_whitelist[9..13]);

        let total_users = u32::from_le_bytes(length_bytes) as usize;


        for i in 0..total_users {
            let start_offset = 13 + (i * 40);

            let user_bytes = &raw_whitelist[start_offset..start_offset + 32];
            let amount_start = start_offset + 32;
            let amount_bytes = &raw_whitelist[amount_start..amount_start + 8];

            if user_bytes == self.source_ata.owner.as_ref() {
                
                let amount_int = u64::from_be_bytes(*amount_bytes)

                if amount_bytes < &amount.to_le_bytes() {
                    panic!("Insufficient Funds")
                }

                user_exists = true;
                break;
            }
        }

        if user_exists == false {

            panic!("user does not exist!");

        }

        Ok(())
    }


    pub fn is_transferring(&mut self) -> Result<()> {
        let token_info = self.source_ata.to_account_info();

        let mut token_data = token_info.try_borrow_mut_data()?;

        let mut account = PodStateWithExtensionsMut::<PodAccount>::unpack(*token_data)?;

        let token_extension = account.get_extension_mut::<TransferHookAccount>()?;

        if !bool::from(token_extension.transferring) {
            panic!("Not Transferring")
        }

        Ok(())
    }

}