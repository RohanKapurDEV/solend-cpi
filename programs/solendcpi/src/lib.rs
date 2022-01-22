use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use solend_utils::SolendDevnet;

pub mod solend_utils;
pub mod utils;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solendcpi {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub payer: Signer<'info>,

    pub solend_devnet: Program<'info, SolendDevnet>,
}
