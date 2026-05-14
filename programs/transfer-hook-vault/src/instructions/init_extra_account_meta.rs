use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;
use spl_tlv_account_resolution::{
    account::ExtraAccountMeta, 
    state::ExtraAccountMetaList};
use spl_transfer_hook_interface::instruction::ExecuteInstruction;

use crate::{ID, WHITELIST_SEED, EXTRA_ACCOUNT_META};

#[derive(Accounts)]
pub struct InitializeAccountMetaList<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    /// CHECK : checked in function below
    #[account(
        init,
        payer = admin,
        space = ExtraAccountMetaList::size_of(
            InitializeAccountMetaList::extra_account_meta()?.len()
        ).unwrap(),
        seeds = [EXTRA_ACCOUNT_META, mint.key().as_ref()],
        bump
    )]
    pub extra_account_meta_list: AccountInfo<'info>,

    pub system_program: Program<'info, System>
}

impl <'info> InitializeAccountMetaList<'info> {
    pub fn init_transfer_hook(&mut self) -> Result<()> {

        let extra_data = Self::extra_account_meta()?;

        ExtraAccountMetaList::init::<ExecuteInstruction>(
            &mut self.extra_account_meta_list.try_borrow_mut_data()?, 
            &extra_data
        ).unwrap();
        Ok(())
    }

    pub fn extra_account_meta() -> Result<Vec<ExtraAccountMeta>> {
        
        let (address, _bump) = Pubkey::find_program_address(
            &[WHITELIST_SEED], 
            &ID
        );
        
        Ok(
            vec![
                ExtraAccountMeta::new_with_pubkey(
                    &address.to_bytes().into(), 
                    false, false
                ).unwrap(),
            ]
        )
    }
}