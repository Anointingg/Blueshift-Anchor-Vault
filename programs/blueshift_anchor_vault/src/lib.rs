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
