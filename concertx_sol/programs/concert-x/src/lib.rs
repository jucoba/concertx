use anchor_lang::prelude::*;

declare_id!("Fh63wv5yhjeNPhyd7jN4ZAhAqLjngHxr8fhV9u7F21fu");

#[program]
pub mod concert_x {
    use super::*;

    pub fn create_concert(
        ctx: Context<CreateConcert>,
        title: String,
        description: String,
        goal_amount: u32,
        start_date: i64,
        end_date: i64,
    ) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        let concert = &mut ctx.accounts.concert;
        concert.creator = ctx.accounts.initializer.key();
        concert.title = title;
        concert.description = description;
        concert.goal_amount = goal_amount;
        concert.start_date = start_date;
        concert.end_date = end_date;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title:String)]
pub struct CreateConcert<'info> {
    #[account(
        init,
        seeds = [b"concertX", initializer.key().as_ref()],
        bump,
        payer = initializer,
        space = DISCRIMINATOR + Concert::INIT_SPACE
    )]
    pub concert: Account<'info, Concert>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Concert {
    pub creator: Pubkey,  
    #[max_len(20)]            // Creator's wallet address
    pub title: String,                // Campaign title
    #[max_len(30)]
    pub description: String,          // Campaign description
    pub goal_amount: u32,             // Funding goal in lamports
    pub current_amount: u64,          // Current amount pledged
    pub start_date: i64,              // Campaign start time
    pub end_date: i64,                // Campaign end time
    pub status: u8,                   // 0 = active, 1 = completed, 2 = cancelled
}

const DISCRIMINATOR: usize = 8;
