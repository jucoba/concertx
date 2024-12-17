use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;
use anchor_lang::system_program::{transfer, Transfer};



declare_id!("Fh63wv5yhjeNPhyd7jN4ZAhAqLjngHxr8fhV9u7F21fu");

#[program]
pub mod concert_x {
    use super::*;

    pub fn create_concert(//maybe we can include checks using requires!()
        ctx: Context<CreateConcert>,
        title: String,
        short_description: String,
        goal_amount: u32,
        ticket_price: u64,
        start_date: i64,
        end_date: i64,
    ) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        let concert = &mut ctx.accounts.concert;
        concert.pda = ctx.accounts.initializer.key();
        concert.title = title;
        concert.short_description = short_description;
        concert.goal_amount = goal_amount;
        concert.ticket_price = ticket_price;
        concert.start_date = start_date;
        concert.end_date = end_date;
        concert.status = 0;
        Ok(())
    }

    

    pub fn make_contribution(ctx: Context<MakeContribution>, amount: u64) -> Result<()> {
        //Require that campaign is active and contribution amount is greater than or equal ticket price
        require!(
            ctx.accounts.concert.status == 0,
            ErrorCode::ConcertNotActive
        );
        require!(
            amount >= ctx.accounts.concert.ticket_price,
            ErrorCode::ContributionAmountTooSmall);

        //Get accounts info and create CPI context for transfer
        let backer_key = ctx.accounts.backer.to_account_info();
        let concert_key = ctx.accounts.concert.to_account_info();
        let program_id = ctx.accounts.system_program.to_account_info();

        let cpi_context = CpiContext::new(
            program_id, 
            Transfer {from: backer_key, to: concert_key},
        );

        //transfer lamports to the concert escrow account
        transfer(cpi_context, amount)?;

        //Update the current amount
        ctx.accounts.concert.current_amount += amount;
        
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title:String)]
pub struct CreateConcert<'info> {
    #[account(
        init,
        seeds = [b"concertX", title.as_bytes(), initializer.key().as_ref()],
        bump,
        payer = initializer,
        space = DISCRIMINATOR + Concert::MAX_SIZE
    )]
    pub concert: Account<'info, Concert>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MakeContribution<'info> {
    #[account(mut)]  // Mutable account to allow lamports transfer
    pub concert: Account<'info, Concert>,  // Escrow account to receive the lamports
    #[account(mut)]  // Mutable account to allow lamports transfer
    pub backer: Signer<'info>,  // The backer account from which funds are deducted
    pub system_program: Program<'info, System>,  // The system program to manage lamport transfers
}

impl Concert {
    pub const MAX_TITLE_LEN: usize = 20;   // Maximum length of the title
    pub const MAX_DESC_LEN: usize = 200;  // Maximum length of the short description
    pub const MAX_SIZE: usize = 32                          // PDA (Pubkey)
                            + 4 + Concert::MAX_TITLE_LEN    // title (4 for length + 20 max characters)
                            + 4 + Concert::MAX_DESC_LEN     // short_description (4 for length + 200 max characters)
                            + 8                             // goal_amount (u64)
                            + 8                             // current_amount (u64)
                            + 8                             // start_date (i64)
                            + 8                             // end_date (i64)
                            + 1;                            // status (u8)
}

#[account]
#[derive(InitSpace)]
pub struct Concert {
    pub pda: Pubkey,  
    #[max_len(20)]                    // Concert PDA, serves as escrow between artist and backer
    pub title: String,                // Campaign title
    #[max_len(100)]
    pub short_description: String,    // Campaign description
    pub goal_amount: u32,             // Funding goal in lamports
    pub ticket_price: u64,             // Ticket price
    pub current_amount: u64,          // Current amount pledged
    pub start_date: i64,              // Campaign start time
    pub end_date: i64,                // Campaign end time
    pub status: u8,                   // 0 = active, 1 = completed, 2 = cancelled
}

const DISCRIMINATOR: usize = 8;

#[error_code]
pub enum ErrorCode {
    #[msg("The concert is not active.")]
    ConcertNotActive,
    #[msg("The funding goal has been exceeded.")]
    GoalExceeded, //Should we allow this as long as the consert has not ended?
    #[msg("Math overflow.")]
    Overflow,
    #[msg("Transfer error")]
    TransferFailed,
    #[msg("Contribution amount is too small")]
    ContributionAmountTooSmall
}
