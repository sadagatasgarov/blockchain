use serde::{Deserialize, Serialize};

/// Transaction present a Bitcoin transaction
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Transaction {
    pub id: String,
    pub vin: Vec<TXInput>,
    pub vout: Vec<TXOutput>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TXInput {
    pub txid: String,
    pub vout: i32,
    pub script_sig: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TXOutput {
    pub value: i32,
    pub script_pub_key:String
}