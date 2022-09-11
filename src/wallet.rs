use {
    serde::{Deserialize, Serialize}
};

use crate::miner::Miner;


// MARK: - JSON Payload (Rest)

#[derive(Debug, Deserialize, Serialize)]
pub struct  Wallet {
    pub address: String, 
    pub club_name: String, 
    pub tottal_hash_rate: i32,
    pub total_shares_mined: i32,
    pub total_workers_online: i32,
    pub workers_online: Vec<Miner>
}


// MARK: - Request body for new wallet

#[derive(Debug, Deserialize, Serialize)]
pub struct NewWalletRequest {
    pub club_name: String
}


// MARK: - Wallet DAO

#[derive(Debug)]
pub struct WalletDAO {
    pub address: String, 
    pub club_name: String
}