use {
   serde::{Serialize, Deserialize}
};


// MARK: - JSON Payload (Rest)

#[derive(Debug, Serialize, Deserialize)]
pub struct Miner {
    pub id: String,
    pub address: String,
    pub club_name: String,
    pub nickname: String, 
    pub hash_rate: i32, // MH/s
    pub shares_mined: i32
}


// MARK: - POST Request body for new miner 

#[derive(Debug, Serialize, Deserialize)]
pub struct NewMinerRequest {
    pub nickname: String
}


// MARK: - DAO

#[derive(Debug)]
pub struct MinerDAO {
    pub id: String,
    pub address: String, 
    pub nichname: String,
    pub hash_rate: i32,  // MH/s
    pub shares_mined: i32
}


