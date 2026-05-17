pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("B6Ko2vrRgrURt9VEoF4Jq6TVpWTCzJjjEsiGic2tZrPV");

#[program]
pub mod transfer_hook_vault {
    use super::*;

    pub fn initialize_whitelist(ctx: Context<InitializeWhitelist>) -> Result<()> {
        ctx.accounts.init(ctx.bumps)
    }

    pub fn add_whitelist(ctx: Context<OpsWhitelist>, user: Pubkey, amount: u64) -> Result<()> {
        ctx.accounts.add_whitelist(user, amount)
    }

    pub fn remove_whitelist(ctx: Context<OpsWhitelist>, user: Pubkey) -> Result<()> {
        ctx.accounts.remove_whitelist(user)
    }

    pub fn init_mint(ctx: Context<MintToken>) -> Result<()> {
        ctx.accounts.init_mint()
    }
    
    pub fn init_distribute(ctx: Context<DistributeToken>, amount: u64) -> Result<()> {
        ctx.accounts.init_distribute(amount)
    }

    pub fn init_transfer_hook(ctx: Context<InitializeAccountMetaList>) -> Result<()> {
        ctx.accounts.init_transfer_hook()
    }

    pub fn execute(ctx: Context<TransferHook>, amount: u64) -> Result<()> {
        ctx.accounts.transfer_hook(amount)
    }

    /* 
    pub fn send_token(ctx: Context<SendToken>, amount: u64) -> Result<()> {
        ctx.accounts.send_token(amount)
    }
    */

    
}
