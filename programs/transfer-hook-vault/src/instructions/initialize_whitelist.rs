use anchor_lang::prelude::*;
use crate::{Whitelist, Config, Balance, 
    CONFIG_SEED, WHITELIST_SEED, BALANCE_SEED
};

#[derive(Accounts)]
pub struct InitializeWhitelist<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = Config::DISCRIMINATOR.len() + Config::INIT_SPACE,
        seeds = [CONFIG_SEED],
        bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        init,
        payer = admin,
        space = Whitelist::DISCRIMINATOR.len() + 4 + 1,
        seeds = [WHITELIST_SEED],
        bump
    )]
    pub whitelist: Account<'info, Whitelist>,

    #[account(
        init,
        payer = admin,
        space = Balance::DISCRIMINATOR.len() + Balance::INIT_SPACE,
        seeds = [BALANCE_SEED, whitelist.key().as_ref()],
        bump
    )]
    pub balance: Account<'info, Balance>,


    pub system_program: Program<'info, System>
}


impl <'info> InitializeWhitelist <'info> {
    pub fn init(&mut self, bumps: InitializeWhitelistBumps) -> Result<()> {

        self.config.set_inner(Config { 
            admin: self.admin.key(),
            bump: bumps.config
        });

        self.whitelist.set_inner(Whitelist { 
            accounts: vec![], 
            bump: bumps.whitelist
        });

        self.balance.set_inner(Balance { 
            amount: 0, 
            bump: bumps.balance
        });

        Ok(())
    }
}

