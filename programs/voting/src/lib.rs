use anchor_lang::prelude::*;

declare_id!("76mwSc7ZmGYsc2j7eoidhLRFiToGCP3eeWZ4CwPMQVvw");

#[program]
pub mod voting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn add_candidate(ctx: Context<AddCandidate>, name: String) -> Result<()> {
        let candidate = &mut ctx.accounts.candidate;
        candidate.name = name;
        candidate.votes = 0;

        msg!("Candidate {} added!", candidate.name);
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, voter_name: String) -> Result<()> {
        let voter = &mut ctx.accounts.voter;
        let candidate = &mut ctx.accounts.candidate;

        voter.name = voter_name;
        voter.has_voted = true;
        
        candidate.votes = candidate.votes.checked_add(1).unwrap();

        msg!("Vote cast successfully for {}!", candidate.name);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct AddCandidate<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 4 + 32 + 8,
        seeds = [b"candidate", name.as_bytes()],
        bump
    )]
    pub candidate: Account<'info, Candidate>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub candidate: Account<'info, Candidate>,
    #[account(
        init, 
        payer = user,
        space = 8 + 4 + 32 + 1, 
        seeds = [b"voter", user.key().as_ref()],
        bump
    )]
    pub voter: Account<'info, Voter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Candidate {
    pub name: String,
    pub votes: u64,
}

#[account]
pub struct Voter {
    pub name: String,
    pub has_voted: bool,
}