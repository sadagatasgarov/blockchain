use crate::errors::Result;
use cli::Cli;
mod block;
mod blockchain;
mod cli;
mod ed25519;
mod errors;
mod transaction;
mod txs;
mod utxoset;

fn main() -> Result<()> {
    let mut cli = Cli::new()?;
    cli.run()?;
    Ok(())
}

//6
