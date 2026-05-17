use anchor_lang::{prelude::*, solana_program::{program::invoke, instruction::{Instruction, AccountMeta}}};
use anchor_spl::{token_interface::{Mint, TokenAccount, TokenInterface}};

#[derive(Accounts)]
pub struct SendToken <'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    /// CHECK:
    pub sender: Signer<'info>,

    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = admin
    )]
    pub source_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = sender
    )]
    pub destination_ata: InterfaceAccount<'info, TokenAccount>,

    /// CHECK:
    pub program_id: AccountInfo<'info>,

    /// CHECK:
    pub whitelist: UncheckedAccount<'info>,

    /// CHECK:
    pub balance: UncheckedAccount<'info>,

    /// CHECK:
    pub extra_account_meta_list: UncheckedAccount<'info>,

    pub token_program: Interface<'info, TokenInterface>
}

impl <'info>SendToken<'info> {
    pub fn send_token(&mut self, amount: u64) -> Result<()> {


        let mut account_metas = vec![
            AccountMeta::new(self.source_ata.key(), false),
            AccountMeta::new_readonly(self.mint.key(), false),
            AccountMeta::new(self.destination_ata.key(), false),
            AccountMeta::new_readonly(self.admin.key(), true),
            AccountMeta::new_readonly(self.extra_account_meta_list.key(), false),
            AccountMeta::new_readonly(self.whitelist.key(), false),
            AccountMeta::new_readonly(self.balance.key(), false),
            AccountMeta::new_readonly(self.program_id.key(), false),
        ];

        let mut data = Vec::with_capacity(18);
        data.push(12);
        data.extend_from_slice(&amount.to_le_bytes());
        data.push(6);

        let ix = Instruction {
            program_id: self.token_program.key(),
            accounts: account_metas,
            data,
        };

        let account_infos = vec![
            self.source_ata.to_account_info(),
            self.mint.to_account_info(),
            self.destination_ata.to_account_info(),
            self.admin.to_account_info(),
            self.extra_account_meta_list.to_account_info(),
            self.whitelist.to_account_info(),
            self.balance.to_account_info(),
            self.program_id.to_account_info(),
        ];

        invoke(&ix, &account_infos)?;

        

        Ok(())
    }
}