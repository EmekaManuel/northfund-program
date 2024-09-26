use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("G12WJ5jg5BUuQtf1H3L4d3WRqNDKHo4RG5qjFHC1NqK2");

#[program]
pub mod northfund_program {

    use super::*;

    pub fn create(
        ctx: Context<Create>,
        name: String,
        description: String,
        target_amount: u64,
        project_url: String,
        progress_update_url: String,
        project_image_url: String,
        category: String,
    ) -> ProgramResult {
        let campaign = &mut ctx.accounts.campaign;
        campaign.name = name;
        campaign.description = description;
        campaign.target_amount = target_amount;
        campaign.project_url = project_url;
        campaign.progress_update_url = progress_update_url;
        campaign.project_image_url = project_image_url;
        campaign.category = category;
        campaign.amount_donated = 0;
        campaign.amount_withdrawn = 0;
        campaign.admin = *ctx.accounts.user.key;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(
        init,
        payer = user,
        space = 9000,
        seeds = [b"CROWDFUND".as_ref(), user.key().as_ref()],
        bump
    )]
    pub campaign: Account<'info, Campaign>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Campaign {
    pub admin: Pubkey,
    pub name: String,
    pub description: String,
    pub target_amount: u64,
    pub project_url: String,
    pub progress_update_url: String,
    pub project_image_url: String,
    pub category: String,
    pub amount_donated: u64,
    pub amount_withdrawn: u64,
}
