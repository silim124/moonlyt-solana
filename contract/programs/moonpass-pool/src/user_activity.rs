use anchor_lang::prelude::*;

#[account]
pub struct UserActivity {
    pub user: Pubkey,               // Identifies the user this activity data belongs to
    pub total_activity_points: u64, // Total points accrued from activities
}

impl UserActivity {
    /// Adds points for a completed activity.
    pub fn add_activity_points(&mut self, points: u64) {
        self.total_activity_points += points;
    }
}

#[derive(Accounts)]
pub struct RecordUserActivity<'info> {
    #[account(mut)]
    pub user_activity: Account<'info, UserActivity>,

    #[account(mut)]
    pub moonpass_pool: Account<'info, crate::moonpass_pool::MoonpassPool>, // Reference to MoonpassPool

    pub user: Signer<'info>,
}

pub fn record_user_activity(ctx: Context<RecordUserActivity>, points: u64) -> Result<()> {
    let user_activity = &mut ctx.accounts.user_activity;
    let moonpass_pool = &mut ctx.accounts.moonpass_pool;

    // Ensure the user is the owner of the user_activity account
    require!(
        user_activity.user == *ctx.accounts.user.key,
        ErrorCode::Unauthorized
    );

    // Update the user's activity points
    user_activity.add_activity_points(points);

    // Optionally update the MoonpassPool with the new activity
    moonpass_pool.update_pool_with_activity(points); // This method needs to be implemented in moonpass_pool.rs

    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized operation.")]
    Unauthorized,
}
