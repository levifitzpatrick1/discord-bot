
mod auth;
mod db;
mod models;



use auth::get_oauth_token;
use dotenvy::dotenv;

use crate::db::db_init::init_db;
use crate::db::db_updates::schedule_data_updates;

#[tokio::main]
async fn main() {
    let _ = dotenv();
    let _ = get_oauth_token().await;
    if let Err(err) = init_db() {
        eprint!("Error initing db: {}", err);
    }
    if let Err(err) = schedule_data_updates().await {
        eprintln!("Error in scheduled update: {}", err);
    }
}
