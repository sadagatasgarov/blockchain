use crate::errors::Result;
use crate::transaction::{hash_pub_key, Transaction};
use bitcoincash_addr::Address;
use clap::builder::Str;
use failure::format_err;
use log::debug;
use serde::de::value;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//TXOutputs collects TXOutput
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TXOutputs {
    pub outputs: Vec<TXOutput>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TXInput {
    pub txid: String,
    pub vout: i32,
    pub signature: Vec<u8>,
    pub pub_key: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TXOutput {
    pub value: i32,
    pub pub_key_hash: Vec<u8>,
}

impl TXInput {
    /// CanUnloclOutputWith checks whether the address initiated the transaction
    pub fn can_unlock_output_with(&self, unlocking_data: &[u8]) -> bool {
        let mut pubkeyhash = self.pub_key.clone();
        hash_pub_key(&mut pubkeyhash);
        pubkeyhash == unlocking_data
    }
}

impl TXOutput {
    // IsLockedWithkEy checks if the output can be unlocked with the provided data
    pub fn is_locked_with_key(&self, pub_key_hash: &[u8]) -> bool {
        self.pub_key_hash == pub_key_hash
    }

    /// Lock signs the putput
    fn lock(&mut self, address: &str) -> Result<()> {
        let pub_key_hash = Address::decode(address).unwrap().body;
        debug!("lock: {}", address);
        self.pub_key_hash = pub_key_hash;
        Ok(())
    }

    pub fn new(value: i32, address: String) -> Result<Self> {
        let mut txo = TXOutput {
            value,
            pub_key_hash: Vec::new(),
        };
        txo.lock(&address)?;
        Ok(txo)
    }
}
