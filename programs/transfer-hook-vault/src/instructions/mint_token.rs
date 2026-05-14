use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::{CONFIG_SEED, Config};

#[derive(Accounts)]
pub struct MintToken <'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [CONFIG_SEED],
        bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        init,
        payer = user,
        mint::decimals = 6,
        mint::authority = user,
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>
}

impl <'info>MintToken<'info> {
    pub fn init_mint(&mut self) -> Result<()> {
        Ok(())
    }
}