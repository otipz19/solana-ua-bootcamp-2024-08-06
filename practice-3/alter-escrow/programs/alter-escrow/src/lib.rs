pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("AY1t5psHSBUX3fn8nve6WqkYS7zLRu1KyLZn61yGvBoc");

#[program]
pub mod alter_escrow {
    use super::*;

    pub fn make_offer(context: Context<MakeOffer>, id: u64, token_a_offered_amount: u64, token_b_wanted_amount: u64) -> Result<()> {
        instructions::make_offer::make_approve(&context, token_a_offered_amount)?;
        instructions::make_offer::save_offer(context, token_a_offered_amount, token_b_wanted_amount, id)
    }

    pub fn take_offer(context: Context<TakeOffer>) -> Result<()> {
        instructions::take_offer::send_token_a_to_taker(&context)?;
        instructions::take_offer::send_token_b_to_maker(&context)
    }
}
