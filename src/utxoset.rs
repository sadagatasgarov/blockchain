use crate::block::Block;
use crate::blockchain::Blockchain;
use crate::errors::Result;
use crate::txs::TXOutputs;
use bincode::deserialize;

/// UTXOSet represents UTXO set
pub struct UTXOSet {
    pub blockchain: Blockchain,
}

impl UTXOSet {
    /// Reindex rebuilds the UTXO set
    pub fn reindex(&self) -> Result<()> {
        std::fs::remove_dir_all("data/utxos")?;
        let db = sled::open("data/utxos")?;

        let utxos = self.blockchain.find_utxo();

        for (txid, outs) in utxos {
            db.insert(txid.as_bytes(), bincode::serialize(&outs)?)?;
        }

        Ok(())
    }

    /// Update updates the UTXO set with transactions from the Block
    ///
    /// The Block is consedered to be the tip of a blockchain
    pub fn update(&self, block: &Block) -> Result<()> {
        let db = sled::open("data/utxos")?;

        for tx in block.get_transaction() {
            if !tx.is_coinbase() {
                for vin in &tx.vin {
                    let mut update_outputs = TXOutputs {
                        outputs: Vec::new(),
                    };
                    let outs: TXOutputs = deserialize(&db.get(&vin.txid)?.unwrap().to_vec())?;
                    for out_idx in 0..outs.outputs.len() {
                        if out_idx != vin.vout as usize {
                            update_outputs.outputs.push(outs.outputs[out_idx].clone());
                        }
                    }

                    if update_outputs.outputs.is_empty() {
                        db.remove(&vin.txid)?;
                    } else {
                        ///buradan davam edecem 08:09 p8
                    }
                }
            }
        }
        Ok(())
    }
}
