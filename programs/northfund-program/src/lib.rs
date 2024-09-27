use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("G12WJ5jg5BUuQtf1H3L4d3WRqNDKHo4RG5qjFHC1NqK2");

#[program]
pub mod northfund_program {

    use anchor_lang::solana_program::instruction::Instruction;

    use super::*;

    // Initialize campaign list
    pub fn initialize_campaign_list(ctx: Context<InitializeCampaignList>) -> ProgramResult {
        let campaign_list = &mut ctx.accounts.campaign_list;
        campaign_list.campaigns = Vec::new();
        Ok(())
    }

    // get campaign data
    pub fn get_campaign_data(ctx: Context<GetCampaign>) -> ProgramResult {
        let student_fund_campaign: &Account<'_, StudentFundCampaign> =
            &ctx.accounts.student_fund_campaign;
        let user: &Signer<'_> = &ctx.accounts.user;
        if student_fund_campaign.admin != *user.key {
            return Err(ProgramError::IncorrectProgramId);
        }
        Ok(())
    }

    // create a new campaign
    pub fn create_student_fund_campaign(
        ctx: Context<CreateStudentFundCampaign>,
        student_name: String,
        matric_number: String,
        course_of_study: String,
        year_of_entry: u16,
        target_amount: u64,
        funding_reason: String,
        student_image_url: String,
    ) -> ProgramResult {
        let student_fund_campaign = &mut ctx.accounts.student_fund_campaign;
        student_fund_campaign.student_name = student_name;
        student_fund_campaign.matric_number = matric_number;
        student_fund_campaign.course_of_study = course_of_study;
        student_fund_campaign.year_of_entry = year_of_entry;
        student_fund_campaign.target_amount = target_amount;
        student_fund_campaign.funding_reason = funding_reason;
        student_fund_campaign.student_image_url = student_image_url;
        student_fund_campaign.amount_donated = 0;
        student_fund_campaign.amount_withdrawn = 0;
        student_fund_campaign.admin = *ctx.accounts.user.key;

        // Add the new campaign to the global campaign list
        let campaign_list = &mut ctx.accounts.campaign_list;
        campaign_list.campaigns.push(student_fund_campaign.key());

        Ok(())
    }

    // withdraw from campaign
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> ProgramResult {
        let student_fund_campaign: &mut Account<'_, StudentFundCampaign> =
            &mut ctx.accounts.student_fund_campaign;
        let user: &mut Signer<'_> = &mut ctx.accounts.user;

        //restricts Withdrawal to campaign admin
        if student_fund_campaign.admin != *user.key {
            return Err(ProgramError::IncorrectProgramId);
        }

        let rent_balance: u64 =
            Rent::get()?.minimum_balance(student_fund_campaign.to_account_info().data_len());

        if **student_fund_campaign.to_account_info().lamports.borrow() - rent_balance < amount {
            return Err(ProgramError::InsufficientFunds);
        }

        **student_fund_campaign
            .to_account_info()
            .try_borrow_mut_lamports()? -= amount;
        **user.to_account_info().try_borrow_mut_lamports()? += amount;

        (&mut ctx.accounts.student_fund_campaign).amount_withdrawn += amount;

        Ok(())
    }

    pub fn donate(ctx: Context<Donate>, amount: u64) -> ProgramResult {
        let ix: Instruction = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.student_fund_campaign.key(),
            amount,
        );

        // Store the result of the invoke function call
        let result = anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.student_fund_campaign.to_account_info(),
            ],
        );

        // Transaction Error Check
        if let Err(e) = result {
            return Err(e.into());
        }

        (&mut ctx.accounts.student_fund_campaign).amount_donated += amount;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeCampaignList<'info> {
    #[account(
        init,
        payer = user,
        space = 9000,
        seeds = [b"CAMPAIGN_LIST"],
        bump
    )]
    pub campaign_list: Account<'info, CampaignList>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CampaignList {
    pub campaigns: Vec<Pubkey>,
}

#[derive(Accounts)]
pub struct CreateStudentFundCampaign<'info> {
    #[account(
        init,
        payer = user,
        space = 9000,
        seeds = [b"StudentFundCampaign".as_ref(), user.key().as_ref()],
        bump
    )]
    pub student_fund_campaign: Account<'info, StudentFundCampaign>,

    #[account(
        mut,
        seeds = [b"CAMPAIGN_LIST"],
        bump
    )]
    pub campaign_list: Account<'info, CampaignList>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub student_fund_campaign: Account<'info, StudentFundCampaign>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetCampaign<'info> {
    #[account(mut)]
    pub student_fund_campaign: Account<'info, StudentFundCampaign>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct Donate<'info> {
    #[account(mut)]
    pub student_fund_campaign: Account<'info, StudentFundCampaign>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetAllCampaigns<'info> {
    #[account(mut)]
    pub campaign_list: Account<'info, CampaignList>,
}

#[account]
pub struct StudentFundCampaign {
    pub admin: Pubkey,
    pub student_name: String,
    pub matric_number: String,
    pub course_of_study: String,
    pub year_of_entry: u16,
    pub funding_reason: String,
    pub student_image_url: String,
    pub target_amount: u64,
    pub amount_donated: u64,
    pub amount_withdrawn: u64,
}
