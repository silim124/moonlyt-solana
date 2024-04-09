use anchor_lang::prelude::*;

#[account]
pub struct MoonpassPool {
    pub owner: Pubkey,
    pub total_activity_points: u64,
    pub total_minted_passes: u64,
    pub total_revenue: u64,
    pub nft_metadata_key: Option<Pubkey>, // Stores only the key, no direct metadata handling
}

impl MoonpassPool {
    /// Initializes a new MoonpassPool with the given owner.
    pub fn initialize(&mut self, owner: Pubkey) {
        self.owner = owner;
        self.total_activity_points = 0;
        self.total_minted_passes = 0;
        self.total_revenue = 0;
        self.nft_metadata_key = None;
    }

    /// Updates the pool with new activity points, minted passes, and revenue.
    pub fn update_pool_data(&mut self, activity_points: u64, minted_passes: u64, revenue: u64) {
        self.total_activity_points = activity_points;
        self.total_minted_passes = minted_passes;
        self.total_revenue = revenue;
    }

    pub fn update_pool_with_activity(&mut self, points: u64) {
        self.total_activity_points += points;
    }

    /// Increases the total revenue by the given amount.
    pub fn update_revenue(&mut self, revenue: u64) {
        self.total_revenue += revenue;
    }

    /// Registers a new NFT mint address with the MoonpassPool.
    /// This function should be called only by the pool owner.
    pub fn register_nft_mint(&mut self, owner: &Signer, mint: Pubkey) -> Result<()> {
        require!(self.owner == *owner.key, MoonpassError::Unauthorized);
        self.nft_metadata_key = Some(mint);
        Ok(())
    }
}

#[error_code]
pub enum MoonpassError {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
}
