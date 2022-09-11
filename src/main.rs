#[macro_use]
extern crate actix_web;

use {
    actix_web::{middleware, HttpServer, App},
    std::{env, io},
};

mod miner;
mod wallet;
mod miner_controller;
mod wallet_controller;
mod util;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    
    HttpServer::new(|| {
        App::new()
        .wrap(middleware::Logger::default())
        .service(miner_controller::list_miners)
        .service(miner_controller::get_miner)
        .service(miner_controller::new_miner)
        .service(wallet_controller::list_wallets)
        .service(wallet_controller::get_wallet)
        .service(wallet_controller::new_wallet)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}
