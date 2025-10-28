use anchor_lang::prelude::*;

declare_id!("6FtZmVscYhGX4riXgpjyL1hs5ZERamm5JAR3GuTYbhXd");

#[program]
pub mod blueshift_anchor_vault {
    use anchor_lang::system_program::{transfer, Transfer};

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn deposit(ctx: Context<VaultAction>, amount: u64) -> Result<()> {
        // deposit logic
        require_eq!(
            ctx.accounts.vault.lamports(),
            0,
            VaultError::VaultAlreadyExist
        );

        require_gt!(
            amount,
            Rent::get()?.minimum_balance(0),
            VaultError::InvalidAmount
        );

        transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.signer.to_account_info(),
                    to: ctx.accounts.vault.to_account_info(),
                },
            ),
            amount,
        )?;

        Ok(())
    }

    pub fn withdraw(ctx: Context<VaultAction>) -> Result<()> {
        // withdraw logic
        require_neq!(ctx.accounts.vault.lamports(), 0, VaultError::InvalidAmount);

        let signer = ctx.accounts.signer.key();
        let signer_seeds = &[b"vault", signer.as_ref(), &[ctx.bumps.vault]];

        transfer(
            CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.vault.to_account_info(),
                    to: ctx.accounts.signer.to_account_info(),
                },
                &[&signer_seeds[..]],
            ),
            ctx.accounts.vault.lamports(),
        )?;

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
        bump // what does bump do and why is it needed?
    )]
    vault: SystemAccount<'info>,
    system_program: Program<'info, System>,
}

#[error_code]
pub enum VaultError {
    #[msg("invalid amount")]
    InvalidAmount,

    #[msg("vault already exist")]
    VaultAlreadyExist,
}
