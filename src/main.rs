use crate::errors::Result;
use cli::Cli;
mod block;
mod blockchain;
mod cli;
mod errors;

fn main() -> Result<()> {
    let mut cli = Cli::new()?;
    cli.run()?;
    Ok(())
}
