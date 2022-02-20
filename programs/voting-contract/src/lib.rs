use anchor_lang::prelude::*;
use std::collections::HashMap;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod voting_contract {
    use super::*;
    pub fn create(ctx: Context<Create>, proposal_names: Vec<String>) -> ProgramResult {
        let ballot = &mut ctx.accounts.ballot;
        ballot.chairperson = *ctx.accounts.user.key;
        let emt :[u8;32] = [0; 32];
        let voter = Voter{
            weight: 1,
            voted: false,
            delegate: Pubkey::new_from_array(emt),
            vote: 0,
        };
        let voters = &mut ballot.voters;
        voters.insert(*ctx.accounts.user.key, voter);
        ballot.proposals.push(Proposal {
            name: String::from("None"),
            vote_count: 0,
        });
        for proposal in proposal_names {
            ballot.proposals.push(Proposal {
                name: proposal,
                vote_count: 0,
            });
        }
        Ok(())
    }
    pub fn giveRightToVote(ctx: Context<GiveRightToVote>, voter: Pubkey) -> ProgramResult {
        let ballot = &mut ctx.accounts.ballot;
        let voters = &ballot.voters;
        assert_eq!(
            ballot.chairperson, *ctx.accounts.user.key,
            "Only Chairperson can Give right to vote"
        );

        match voters.get(&voter){
            Some(voter_) => {
                assert_eq!(
                    voter_.voted, false,
                    "The voter already voted"
                );
                assert_eq!(voter_.weight, 0);
            },
            None => println!("Invalid")
        };
        let emt :[u8;32] = [0; 32];
        let voter_ = Voter{
            weight: 1,
            voted: false,
            delegate: Pubkey::new_from_array(emt),
            vote: 0,
        };
        let voters = &mut ballot.voters;
        voters.insert(voter, voter_);
        Ok(())
    }

    pub fn delegate(ctx: Context<Delegate>, to: Pubkey) -> ProgramResult {
        let ballot = &mut ctx.accounts.ballot;
        let mut weight = 0;
        {
            let voters = &ballot.voters;
            match voters.get(ctx.accounts.user.key){
                Some(v) => {
                    assert_eq!(v.voted, false);
                    weight = v.weight;
                },
                None => {
                    println!("Invalid")
                }
            };
        }
        assert_ne!(to, *ctx.accounts.user.key, "Self-delegation is disallowed");
        let mut voted = false;
        let mut vote_:u8 = 0;
        {
            let voters = &mut ballot.voters;
            match voters.get(&to){
                Some(delegate_) => {
                    if delegate_.voted {
                        voted = true;
                        vote_ = delegate_.vote;
                    }
                },
                None => {
                    println!("Invalid")
                }
            };
        }
        {
            let proposals = &mut ballot.proposals;
            if voted{
                proposals[vote_ as usize].vote_count += weight;
            }
        }

        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, proposal :u8) -> ProgramResult {
        let ballot = &mut ctx.accounts.ballot;
        let mut weight : u8 = 0;
        {
            let voters = &mut ballot.voters;
            match voters.get(ctx.accounts.user.key){
                Some(sender) => {
                    weight=sender.weight;
                    assert_ne!(sender.weight,0,"Has no right to vote");
                    assert_eq!(sender.voted, false,"Already voted");
                },
                None => {
                    println!("Invalid")
                }
            };
            let emt :[u8;32] = [0; 32];
            let voter_ = Voter{
                weight: 1,
                voted: true,
                delegate: Pubkey::new_from_array(emt),
                vote: proposal,
            };
            voters.insert(*ctx.accounts.user.key, voter_);
        }
        ballot.proposals[proposal as usize].vote_count += weight;
        Ok(())
    }

}

#[account]
pub struct Ballot {
    pub chairperson: Pubkey,
    pub proposals: Vec<Proposal>,
    pub voters: HashMap<Pubkey, Voter>,
}

#[account]
pub struct Voter {
    pub weight: u8,
    pub voted: bool,
    pub delegate: Pubkey,
    pub vote: u8,
}

#[account]
pub struct Proposal {
    pub name: String,
    pub vote_count: u8,
}

#[derive(Accounts)]
pub struct GiveRightToVote<'info> {
    #[account(mut)]
    pub ballot: Account<'info, Ballot>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct Delegate<'info> {
    #[account(mut)]
    pub ballot: Account<'info, Ballot>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub ballot: Account<'info, Ballot>,
    #[account(mut)]
    pub user: Signer<'info>,
}


#[derive(Accounts)]
pub struct WinningProposal<'info> {
    #[account(mut)]
    pub ballot: Account<'info, Ballot>,
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 8 + 64 + 64 + 64 + 64)]
    pub ballot: Account<'info, Ballot>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
