use {
    actix_web::{HttpResponse},
    actix_web::web::Json,
    crate::miner::*,
    crate::util::*,
};

// MARK: - List all miners

#[get("/miners")]
pub async fn list_miners() -> HttpResponse {  
    // TODO:  get all from miners db 
    let miners: Vec<Miner> = vec![];

    ResponseType::Ok(miners).get_response()
}

// MARK: - Get miner by id

#[get("/miners/{id}")]
pub async fn get_miner() -> HttpResponse {
    // TODO: get miner from id 
    let miner: Option<Miner> = None;
    
    match miner {
        Some(miner) => ResponseType::Ok(miner).get_response(),
        None => ResponseType::NotFound(
            NotFoundMessage::new("Miner not found".to_string())
        ).get_response()
    }
}

// MARK: - Create new miner

#[post("/wallets/{id}/miner")]
pub async fn new_miner(miner_request: Json<NewMinerRequest>) -> HttpResponse {
    // TODO: create new miner object and save to wallet db
    let miners: Vec<Miner>= vec![];
    ResponseType::Created(miners).get_response()
}