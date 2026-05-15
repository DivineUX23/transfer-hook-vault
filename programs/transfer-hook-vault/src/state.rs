use anchor_lang::prelude::*;

#[derive(InitSpace)]
#[account]
pub struct Config {
    pub admin: Pubkey,
    pub bump: u8
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct WhitelistEntry {
    pub user: Pubkey,
    pub amount: u64,
}

#[account]
pub struct Whitelist {
    pub bump: u8,
    pub accounts: Vec<WhitelistEntry>
}

#[derive(InitSpace)]
#[account]
pub struct Balance {
    pub amount: u64,
    pub bump: u8,
}
