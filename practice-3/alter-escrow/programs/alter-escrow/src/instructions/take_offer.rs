use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, TransferChecked},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::Offer;

#[derive(Accounts)]
pub struct TakeOffer<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(mut)]
    pub maker: SystemAccount<'info>,

    #[account(mint::token_program = token_program)]
    pub token_mint_a: InterfaceAccount<'info, Mint>,

    #[account(mint::token_program = token_program)]
    pub token_mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = maker, // or offer?
        associated_token::token_program = token_program,
    )]
    pub maker_token_account_a: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = token_mint_b,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_token_account_b: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = token_mint_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    pub taker_token_account_a: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    pub taker_token_account_b: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        close = taker,
        has_one = maker,
        has_one = token_mint_a,
        has_one = token_mint_b,
    )]
    pub offer: Account<'info, Offer>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn send_token_b_to_maker(context: &Context<TakeOffer>) -> Result<()> {
    let transfer_accounts = TransferChecked {
        authority: context.accounts.taker.to_account_info(),
        from: context.accounts.taker_token_account_b.to_account_info(),
        to: context.accounts.maker_token_account_b.to_account_info(),
        mint: context.accounts.token_mint_b.to_account_info(),
    };

    let cpi_context = CpiContext::new(
        context.accounts.token_program.to_account_info(),
        transfer_accounts,
    );

    transfer_checked(
        cpi_context,
        context.accounts.offer.token_b_wanted_amount,
        context.accounts.token_mint_b.decimals,
    )
}

pub fn send_token_a_to_taker(context: &Context<TakeOffer>) -> Result<()> {
    let transfer_accounts = TransferChecked {
        authority: context.accounts.offer.to_account_info(),
        from: context.accounts.maker_token_account_a.to_account_info(),
        to: context.accounts.taker_token_account_a.to_account_info(),
        mint: context.accounts.token_mint_a.to_account_info(),
    };

    let signer_seeds: [&[&[u8]]; 1] = [&[
        b"offer",
        context.accounts.maker.to_account_info().key.as_ref(),
        &context.accounts.offer.id.to_le_bytes()[..],
        &[context.accounts.offer.bump],
    ]];

    let cpi_context = CpiContext::new_with_signer(
        context.accounts.token_program.to_account_info(),
        transfer_accounts,
        &signer_seeds
    );

    transfer_checked(
        cpi_context,
        context.accounts.offer.token_a_delegated_amount,
        context.accounts.token_mint_a.decimals,
    )
}
