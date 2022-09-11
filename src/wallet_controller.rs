use {
    actix_web::{HttpResponse},
    actix_web::web::Json,

    crate::wallet::*,
    crate::util::*
};

#[get("wallets/")]
pub async fn list_wallets() -> HttpResponse {
    // TODO: get all wallets
    let wallets: Vec<Wallet> = vec![];
    ResponseType::Ok(wallets).get_response()
}

#[get("wallets/{id}")]
pub async fn get_wallet() -> HttpResponse {
    // TODO: get wallet by id
    let wallet: Option<Wallet> = None;

    match wallet {
        Some(wallet) => ResponseType::Ok(wallet).get_response(),
        None => ResponseType::NotFound(
            NotFoundMessage::new("Wallet not found".to_string())
        ).get_response()
    }
}

#[post("wallets/}")]
pub async fn new_wallet() -> HttpResponse {
    // TODO: get all wallets
    let wallets: Vec<Wallet> = vec![];
    ResponseType::Ok(wallets).get_response()
}