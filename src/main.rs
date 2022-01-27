#![feature(proc_macro_hygiene, decl_macro)]

#[cfg(test)]
mod main_test;

mod db;
mod error;
mod types;
mod helpers;
mod routes;

use db::*;

use std::error::Error;
use mongodb::Database;
use tokio;

#[macro_use]
extern crate rocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let db = db_connection().await?;
    rocket(db).launch().await?;    
    Ok(())
}

fn rocket(db: Database) -> rocket::Rocket<rocket::Build> {
    rocket::build()
    .mount("/api", routes::get_routes())
        .manage(db)
    
}
