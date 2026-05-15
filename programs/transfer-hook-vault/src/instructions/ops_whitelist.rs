// UncheckedAccount FOR WHITELIST WAS USED BECAUSE OF THE LEARNING EXPRIENCE. well I'm actually jobless so I have time for this

use anchor_lang::{prelude::*, system_program};
use crate::{Config, CONFIG_SEED, BALANCE_SEED, Balance};

#[derive(Accounts)]
pub struct OpsWhitelist<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        has_one = admin,
        seeds = [CONFIG_SEED],
        bump
    )]
    pub config: Account<'info, Config>,

    /// CHECK: checks are done manually in the code below
    #[account(mut)]
    pub whitelist: UncheckedAccount<'info>,

    #[account(
        seeds = [BALANCE_SEED, whitelist.key().as_ref()],
        bump
    )]
    pub balance: Account<'info, Balance>,

    pub system_program: Program<'info, System>

}


impl <'info>OpsWhitelist<'info> {

    pub fn add_whitelist(&mut self, user: Pubkey, amount: u64) -> Result<()> {

        let total_users: usize;

        {
            let raw_whitelist = self.whitelist.try_borrow_data()?;

            let mut length_bytes = [0u8; 4];

            length_bytes.copy_from_slice(&raw_whitelist[9..13]);

            total_users = u32::from_le_bytes(length_bytes) as usize;


            for i in 0..total_users {
                let start_offset = 13 + (i * 40);

                let user_bytes = &raw_whitelist[start_offset..start_offset + 32];

                if user_bytes == user.as_ref() {
                    panic!("user already exist!")
                }
            }
        }  

        // increase size;
        self.realloc_whitelist(true)?;

        let mut raw_whitelist = self.whitelist.try_borrow_mut_data()?;

        let start  = 13 + (total_users * 40);
        let end = start + 40;

        /*let data = WhitelistEntry {
            user: user,
            amount: amount
        };*/

        let mut data_bytes = [0u8; 40];       
        data_bytes[..32].copy_from_slice(user.as_ref());
        data_bytes[32..].copy_from_slice(&amount.to_le_bytes());

        raw_whitelist[start..end].copy_from_slice(&data_bytes);
        let new_total = (total_users + 1) as u32;
        raw_whitelist[9..13].copy_from_slice(&new_total.to_le_bytes());


        // update balance:

        self.balance.amount += amount;

        Ok(())
    }




    pub fn remove_whitelist(&mut self, user: Pubkey) -> Result<()> {

        {
            let mut user_exists = false;

            let mut raw_whitelist = self.whitelist.try_borrow_mut_data()?;

            let mut length_bytes  = [0u8; 4];

            length_bytes.copy_from_slice(&raw_whitelist[9..13]);

            let mut total_users = u32::from_le_bytes(length_bytes);


            for i in 0..(total_users as usize) {

                let start_offset = 13 + (i * 40);
                let user_bytes = &raw_whitelist[start_offset..start_offset + 32];

                let last_end  = 13 + (total_users * 40) as usize;
                let last_start = last_end - 40 as usize;

                if user_bytes == user.as_ref() {
                    {
                        // remove from balance first:
                        let mut user_balance_bytes = [0u8; 8];
                        user_balance_bytes.copy_from_slice(&raw_whitelist[start_offset + 32..start_offset + 40]);

                        let user_balance: u64 =  u64::from_le_bytes(user_balance_bytes);

                        self.balance.amount -= user_balance;
                    }

                    if user_bytes != &raw_whitelist[last_start..last_start + 32] {
                        
                        let mut last_data = [0u8; 40];

                        last_data.copy_from_slice(&raw_whitelist[last_start..last_end]);

                        raw_whitelist[start_offset..start_offset + 40].copy_from_slice(&last_data);

                    };

                    total_users -= 1;

                    let new_total_users = total_users.to_le_bytes();

                    raw_whitelist[9..13].copy_from_slice(&new_total_users);

                    user_exists = true;
                    break;
                }
            }

            if user_exists == false {

                panic!("user does not exist!");

            }
        }

        // reduce size;
        self.realloc_whitelist(false)?;

        Ok(())
    }



    pub fn realloc_whitelist(&self, is_add: bool) -> Result<()> {

        let whitelist_info = self.whitelist.to_account_info();

        if is_add == true {

            let new_size = whitelist_info.data_len() + 40;

            let lamports_req = (Rent::get()?).minimum_balance(new_size);

            let rent_diff = lamports_req - whitelist_info.lamports();

            let cpi_program = self.system_program.key();
            let cpi_accounts = system_program::Transfer {
                from: self.admin.to_account_info(),
                to: whitelist_info.to_account_info()
            };

            let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
            system_program::transfer(cpi_context, rent_diff)?;

            whitelist_info.resize(new_size)?;

        } else {

            let new_size = whitelist_info.data_len() - 40;

            let lamports_req = (Rent::get()?).minimum_balance(new_size);

            let rent_diff = whitelist_info.lamports() - lamports_req;

            let mut whitelist_lamport = whitelist_info.try_borrow_mut_lamports()?;
            let mut admin_lamport = self.admin.try_borrow_mut_lamports()?;

            **whitelist_lamport -= rent_diff;
            **admin_lamport += rent_diff;

            whitelist_info.resize(new_size)?;

        }


        Ok(())
    }


}