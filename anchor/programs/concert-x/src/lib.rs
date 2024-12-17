use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

// Program ID for the ConcertX smart contract
declare_id!("Fh63wv5yhjeNPhyd7jN4ZAhAqLjngHxr8fhV9u7F21fu");

// Main program module containing all instruction handlers
#[program]
pub mod concert_x {
    use super::*;

    pub fn create_concert(
    // Creates a new concert crowdfunding campaign
    //
    // # Arguments
    // * `ctx` - The context of the instruction
    // * `title` - The title of the concert campaign
    // * `short_description` - Brief description of the concert
    // * `goal_amount` - Target funding amount in lamports
    // * `start_date` - Unix timestamp for campaign start
    // * `end_date` - Unix timestamp for campaign end
        ctx: Context<CreateConcert>,
        title: String,
        short_description: String,
        goal_amount: u32,
        ticket_price: u64,
        start_date: i64,
        end_date: i64,
    ) -> Result<()> {
        msg!("Creating new concert campaign");
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
pub struct MakeContribution<'info> {
    #[account(mut)]  /// The concert account receiving the contribution
    pub concert: Account<'info, Concert>,  // Escrow account to receive the lamports
    #[account(mut)]  /// The account making the contribution
    pub backer: Signer<'info>,
    pub system_program: Program<'info, System>,  // The system program to manage lamport transfers
}

/// Constants and size calculations for the Concert account
impl Concert {
    /// Maximum length allowed for concert title
    pub const MAX_TITLE_LEN: usize = 20;
    /// Maximum length allowed for concert description
    pub const MAX_DESC_LEN: usize = 200;
    /// Total size of the Concert account struct in bytes
    pub const MAX_SIZE: usize = 32                          // PDA (Pubkey)
                            + 4 + Concert::MAX_TITLE_LEN    // title (length prefix + chars)
                            + 4 + Concert::MAX_DESC_LEN     // short_description (length prefix + chars)
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
    /// Concert PDA, serves as escrow between artist and backer
    pub pda: Pubkey,
    /// The title of the concert campaign
    #[max_len(20)]                    
    pub title: String,
    /// Brief description of the concert
    #[max_len(100)]
    pub short_description: String,
    /// Target funding amount in lamports
    pub goal_amount: u32,
    pub ticket_price: u64,
    /// Amount of lamports currently raised
    pub current_amount: u64,     
    /// Unix timestamps when the campaign starts
    pub start_date: i64,      
    /// Unix timestamps when the campaign ends
    pub end_date: i64,               
    /// 0 = active, 1 = completed, 2 = cancelled
    pub status: u8,                  
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
    #[msg("Contribution amount is too small")]
    ContributionAmountTooSmall
}
