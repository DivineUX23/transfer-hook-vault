use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, MintTo, TokenAccount, TokenInterface, mint_to}};

use crate::{CONFIG_SEED, Config};

#[derive(Accounts)]
pub struct DistributeToken <'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub destination_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [CONFIG_SEED],
        bump
    )]
    pub config: Account<'info, Config>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>
}

impl <'info>DistributeToken<'info> {
    pub fn init_distribute(&mut self, amount: u64) -> Result<()> {

        let cpi_accounts = MintTo {
            mint: self.mint.to_account_info(),
            to: self.destination_ata.to_account_info(),
            authority: self.user.to_account_info()
        };

        let cpi_program = self.token_program.key();

        let cpi_contex = CpiContext::new(cpi_program, cpi_accounts);
        
        mint_to(cpi_contex, amount)?;

        Ok(())
    }
}