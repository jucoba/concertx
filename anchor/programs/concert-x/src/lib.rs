use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;



declare_id!("Fh63wv5yhjeNPhyd7jN4ZAhAqLjngHxr8fhV9u7F21fu");

#[program]
pub mod concert_x {
    use super::*;

    pub fn create_concert(
        ctx: Context<CreateConcert>,
        title: String,
        short_description: String,
        goal_amount: u32,
        start_date: i64,
        end_date: i64,
        max_token_supply: u32,
    ) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        let concert = &mut ctx.accounts.concert;
        concert.creator = ctx.accounts.initializer.key();
        concert.title = title;
        concert.short_description = short_description;
        concert.goal_amount = goal_amount;
        concert.start_date = start_date;
        concert.end_date = end_date;
        concert.max_token_supply = max_token_supply;
        Ok(())
    }

    

    pub fn make_aportation(ctx: Context<MakeAportation>, amount: u64) -> Result<()> {
        
        
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
pub struct MakeAportation<'info> {
    #[account(mut)]  // Mutable account to allow lamports transfer
    pub concert: Account<'info, Concert>,  // Concert account to receive the lamports
    #[account(mut)]  // Mutable account to allow lamports transfer
    pub backer: Signer<'info>,  // The backer account from which funds are deducted
    pub system_program: Program<'info, System>,  // The system program to manage lamport transfers
}




impl Concert {
    pub const MAX_TITLE_LEN: usize = 20;   // Maximum length of the title
    pub const MAX_DESC_LEN: usize = 200;  // Maximum length of the short description
    pub const MAX_SIZE: usize = 32                          // creator (Pubkey)
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
    pub creator: Pubkey,  
    #[max_len(20)]            // Creator's wallet address
    pub title: String,                // Campaign title
    #[max_len(100)]
    pub short_description: String,          // Campaign description
    pub goal_amount: u32,             // Funding goal in lamports
    pub current_amount: u64,          // Current amount pledged
    pub start_date: i64,              // Campaign start time
    pub end_date: i64,                // Campaign end time
    pub status: u8,                   // 0 = active, 1 = completed, 2 = cancelled
    pub max_token_supply: u32,
}

const DISCRIMINATOR: usize = 8;

#[error_code]
pub enum ErrorCode {
    #[msg("The concert is not active.")]
    ConcertNotActive,
    #[msg("The funding goal has been exceeded.")]
    GoalExceeded,
    #[msg("Math overflow.")]
    Overflow,
    #[msg("Transfer error")]
    TransferFailed,
}
