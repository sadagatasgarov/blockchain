use crate::errors::Result;
use cli::Cli;
mod block;
mod blockchain;
mod cli;
mod ed25519;
mod errors;
mod transaction;
mod txs;

fn main() -> Result<()> {
    let mut cli = Cli::new()?;
    cli.run()?;
    Ok(())
}

//5 5.03
