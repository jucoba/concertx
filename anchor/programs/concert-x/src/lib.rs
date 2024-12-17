use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

/// Program ID for the ConcertX smart contract
declare_id!("Fh63wv5yhjeNPhyd7jN4ZAhAqLjngHxr8fhV9u7F21fu");

/// Main program module containing all instruction handlers
#[program]
pub mod concert_x {
    use super::*;

    /// Creates a new concert crowdfunding campaign
    /// 
    /// # Arguments
    /// * `ctx` - The context of the instruction
    /// * `title` - The title of the concert campaign
    /// * `short_description` - Brief description of the concert
    /// * `goal_amount` - Target funding amount in lamports
    /// * `start_date` - Unix timestamp for campaign start
    /// * `end_date` - Unix timestamp for campaign end
    /// * `max_token_supply` - Maximum number of tokens that can be minted
    pub fn create_concert(
        ctx: Context<CreateConcert>,
        title: String,
        short_description: String,
        goal_amount: u32,
        start_date: i64,
        end_date: i64,
        max_token_supply: u32,
    ) -> Result<()> {
        msg!("Creating new concert campaign");
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

    /// Processes a backer's contribution to a concert campaign
    /// 
    /// # Arguments
    /// * `ctx` - The context of the instruction
    /// * `amount` - The amount of lamports to contribute
    pub fn make_aportation(ctx: Context<MakeAportation>, amount: u64) -> Result<()> {
        Ok(())
    }
}

/// Account validation struct for creating a new concert
#[derive(Accounts)]
#[instruction(title:String)]
pub struct CreateConcert<'info> {
    /// The concert account to be created
    #[account(
        init,
        seeds = [b"concertX", title.as_bytes(), initializer.key().as_ref()],
        bump,
        payer = initializer,
        space = DISCRIMINATOR + Concert::MAX_SIZE
    )]
    pub concert: Account<'info, Concert>,
    /// The account creating the concert (pays for rent)
    #[account(mut)]
    pub initializer: Signer<'info>,
    /// The system program
    pub system_program: Program<'info, System>,
}

/// Account validation struct for making contributions
#[derive(Accounts)]
pub struct MakeAportation<'info> {
    /// The concert account receiving the contribution
    #[account(mut)]
    pub concert: Account<'info, Concert>,
    /// The account making the contribution
    #[account(mut)]
    pub backer: Signer<'info>,
    /// The system program for handling transfers
    pub system_program: Program<'info, System>,
}

/// Constants and size calculations for the Concert account
impl Concert {
    /// Maximum length allowed for concert title
    pub const MAX_TITLE_LEN: usize = 20;
    /// Maximum length allowed for concert description
    pub const MAX_DESC_LEN: usize = 200;
    /// Total size of the Concert account struct in bytes
    pub const MAX_SIZE: usize = 32                          // creator (Pubkey)
                            + 4 + Concert::MAX_TITLE_LEN    // title (length prefix + chars)
                            + 4 + Concert::MAX_DESC_LEN     // description (length prefix + chars)
                            + 8                             // goal_amount
                            + 8                             // current_amount
                            + 8                             // start_date
                            + 8                             // end_date
                            + 1;                            // status
}

/// Main account structure for storing concert campaign data
#[account]
#[derive(InitSpace)]
pub struct Concert {
    /// The wallet address of the concert creator
    pub creator: Pubkey,
    /// The title of the concert campaign
    #[max_len(20)]            
    pub title: String,
    /// Brief description of the concert
    #[max_len(200)]
    /// Target funding amount in lamports
    pub goal_amount: u32,
    /// Amount of lamports currently raised
    pub current_amount: u64,
    /// Unix timestamp when the campaign starts
    pub start_date: i64,
    /// Unix timestamp when the campaign ends
    pub end_date: i64,
    /// Campaign status: 0 = active, 1 = completed, 2 = cancelled
    pub status: u8,
    /// Maximum number of tokens that can be minted
    pub max_token_supply: u32,
}

/// Size of the account discriminator
const DISCRIMINATOR: usize = 8;

/// Custom error codes for the program
#[error_code]
pub enum ErrorCode {
    /// Returned when trying to interact with an inactive concert
    #[msg("The concert is not active.")]
    ConcertNotActive,
    /// Returned when contribution would exceed the funding goal
    #[msg("The funding goal has been exceeded.")]
    GoalExceeded,
    /// Returned when a calculation would cause an overflow
    #[msg("Math overflow.")]
    Overflow,
    /// Returned when a lamport transfer fails
    #[msg("Transfer error")]
    TransferFailed,
}
