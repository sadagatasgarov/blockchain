use clap::{arg, Command};

use crate::blockchain::Blockchain;
use crate::errors::Result;

pub struct Cli {
    bc: Blockchain,
}

impl Cli {
    pub fn new() -> Result<Cli> {
        Ok(Cli {
            bc: Blockchain::new()?,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let matches = Command::new("blockchain-rust-demo")
            .version("0.1")
            .author("sada@gmail.com")
            .subcommand(Command::new("printchain").about("print all the chain bolcks"))
            .subcommand(
                Command::new("addblock")
                    .about("add a block in the blockchain")
                    .arg(arg!(<DATA>" 'the blockchain data'")),
            )
            .get_matches();
        if let Some(ref matches) = matches.subcommand_matches("addblock") {
            if let Some(c) = matches.get_one::<String>("DATA") {
                self.add_block(String::from(c))?;
            } else {
                println!("Not printing testing lists...");
            }
        }

        if let Some(_) = matches.subcommand_matches("printchain") {
            self.print_chain();
        }

        Ok(())
    }

    fn add_block(&mut self, data: String) -> Result<()> {
        self.bc.add_block(data)
    }

    fn print_chain(&mut self) -> Result<()> {
        Ok(for b in &mut self.bc.iter() {
            println!("block: {:#?}", b);
        })
    }
}
