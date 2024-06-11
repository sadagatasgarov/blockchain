use crate::errors::Result;
use cli::Cli;
mod block;
mod blockchain;
mod cli;
mod errors;
mod transaction;

fn main() -> Result<()> {
    let mut cli = Cli::new()?;
    cli.run()?;
    Ok(())
}
