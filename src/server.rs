use std::{
    collections::{HashMap, HashSet},
    io::Read,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use failure::format_err;
use log::info;
use serde::{Deserialize, Serialize};

use crate::errors::Result;
use crate::{block::Block, transaction::Transaction, utxoset::UTXOSet};

const KNOWN_NODE1: &str = "localhost:3000";
const CMD_LEN: usize = 12;
const VERSION: i32 = 1;

pub struct Server {
    node_address: String,
    mining_address: String,
    inner: Arc<Mutex<ServerInner>>,
}

struct ServerInner {
    known_nodes: HashSet<String>,
    utxo: UTXOSet,
    blocks_in_transit: Vec<String>,
    mempool: HashMap<String, Transaction>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Blockmsg {
    add_from: String,
    block: Block,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GetBlockmsg {
    add_from: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GetDatamsg {
    add_from: String,
    kind: String,
    id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Invmsg {
    add_from: String,
    kind: String,
    items: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Txmsg {
    add_from: String,
    transaction: Transaction,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct Versionmsg {
    add_from: String,
    version: i32,
    best_height: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum Message {
    Addr(Vec<String>),
    Version(Versionmsg),
    Tx(Txmsg),
    GetData(GetDatamsg),
    GetBlock(GetBlockmsg),
    Inv(Invmsg),
    Block(Blockmsg),
}

impl Server {
    pub fn new(port: &str, miner_address: &str, utxo: UTXOSet) -> Result<Server> {
        let mut node_set = HashSet::new();
        node_set.insert(String::from(KNOWN_NODE1));
        Ok(Server {
            node_address: String::from("localhost") + port,
            mining_address: miner_address.to_string(),
            inner: Arc::new(Mutex::new(ServerInner {
                known_nodes: node_set,
                utxo,
                blocks_in_transit: Vec::new(),
                mempool: HashMap::new(),
            })),
        })
    }

    fn handle_connnection(&self, mut stream: TcpStream) -> Result<()> {
        let mut buffer = Vec::new();
        let count = stream.read_to_end(&mut buffer)?;
        info!("Accept request: length {}", count);

        let cmd = bytes_to_cmd(&buffer)?;

        match cmd {
            Message::Addr(data) => self.handle_addr(data)?,
            Message::Block(data) => self.handle_block(data)?,
            Message::Inv(data) => self.handle_inv(data)?,
            Message::GetBlock(data) => self.handle_get_blocks(data)?,
            Message::GetData(data) => self.handle_get_data(data)?,
        }
    }
}

fn bytes_to_cmd(bytes: &[u8]) -> Result<Message> {
    let mut cmd = Vec::new();
    let cmd_bytes = &bytes[..CMD_LEN];
    let data = &bytes[CMD_LEN..];
    for b in cmd_bytes {
        if 0 as u8 != *b {
            cmd.push(*b)
        }
    }
    info!("cmd: {}", String::from_utf8(cmd.clone())?);

    if cmd == "addr".as_bytes() {
        let data: Vec<String> = bincode::deserialize(data)?;
        Ok(Message::Addr(data))
    } else if cmd == "block".as_bytes() {
        let data: Blockmsg = bincode::deserialize(data)?;
        Ok(Message::Block(data))
    } else if cmd == "inv".as_bytes() {
        let data: Invmsg = bincode::deserialize(data)?;
        Ok(Message::Inv(data))
    } else if cmd == "getblocks".as_bytes() {
        let data: GetBlockmsg = bincode::deserialize(data)?;
        Ok(Message::GetBlock(data))
    } else if cmd == "getdata".as_bytes() {
        let data: GetDatamsg = bincode::deserialize(data)?;
        Ok(Message::GetData(data))
    } else if cmd == "tx".as_bytes() {
        let data: Txmsg = bincode::deserialize(data)?;
        Ok(Message::Tx(data))
    } else if cmd == "version".as_bytes() {
        let data: Versionmsg = bincode::deserialize(data)?;
        Ok(Message::Version(data))
    } else {
        Err(format_err!("Unknown command in the server"))
    }


}


// 10.13