use anchor_lang::prelude::*;

declare_id!("53hFFhXaWTZSjWK9fXFP4HdH3BxF9RpcAj3pB7jFakGV");

#[program]
pub mod front_stike {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
