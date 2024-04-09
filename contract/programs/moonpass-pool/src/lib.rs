use anchor_lang::prelude::*;

// Module declarations for organization
pub mod moonpass_pool;
pub mod user_activity;

use moonpass_pool::MoonpassPool;

declare_id!("BVDNeigfTAwkN3ixdymyxK1oWDf9KPYQVJQDJ6vKAoGy");

#[program]
pub mod moonlyt_program {
    use super::*;

    pub fn initialize_pool(ctx: Context<InitializePool>, owner: Pubkey) -> Result<()> {
        let moonpass_pool = &mut ctx.accounts.moonpass_pool;
        moonpass_pool.initialize(owner);
        Ok(())
    }

}

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(init, payer = owner, space = 8 + 256, seeds = [b"moonpass", owner.key().as_ref()], bump)]
    pub moonpass_pool: Account<'info, MoonpassPool>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
