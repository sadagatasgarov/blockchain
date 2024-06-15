use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TXInput {
    pub txid: String,
    pub vout: i32,
    pub script_sig: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TXOutput {
    pub value: i32,
    pub script_pub_key: String,
}

impl TXInput {
    /// CanUnloclOutputWith checks whether the address initiated the transaction
    pub fn can_unlock_output_with(&self, unlocking_data: &str) -> bool {
        self.script_sig == unlocking_data
    }
}

impl TXOutput {
    // CanBeUnlockedWith checks if the output can be unlocked with the provided data
    pub fn can_be_unlock_with(&self, unlocking_data: &str) -> bool {
        self.script_pub_key == unlocking_data
    }
}
