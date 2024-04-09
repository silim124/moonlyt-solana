use anchor_lang::prelude::*;
use crate::moonpass_pool::MoonpassPool;

#[derive(Accounts)]
pub struct DistributeRevenue<'info> {
    #[account(mut)]
    pub moonpass_pool: Account<'info, MoonpassPool>,
    pub system_program: Program<'info, System>,
}

pub fn distribute_revenue(ctx: Context<DistributeRevenue>, total_revenue: u64) -> Result<()> {
    let moonpass_pool = &mut ctx.accounts.moonpass_pool;
    let creator_share = (total_revenue * 70) / 100;
    let remaining_revenue = total_revenue - creator_share;
    let governance_share = (remaining_revenue * 5) / 100;
    let investor_share = remaining_revenue - governance_share;

    msg!("Creator share: {}", creator_share);
    msg!("Investor share: {}", investor_share);
    msg!("Governance share: {}", governance_share);

    Ok(())
}
