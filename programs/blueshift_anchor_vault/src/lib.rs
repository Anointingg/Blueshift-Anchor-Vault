use anchor_lang::prelude::*;

declare_id!("6FtZmVscYhGX4riXgpjyL1hs5ZERamm5JAR3GuTYbhXd");

#[program]
pub mod blueshift_anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct VaultAction<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", signer.key().as_ref()],
        bump
    )]
    vault: SystemAccount<'info>,
    system_program: Program<'info, System>,
}

#[error_code]
pub enum VaultError {
    #[msg("invalid amount")]
    InvalidAmount,

    #[msg("vault already exist")]
    VaultAlreadyExist
}