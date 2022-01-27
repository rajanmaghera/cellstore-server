use mongodb::{Client, Database, IndexModel, bson::doc, options::{ClientOptions, IndexOptions}};

use crate::types::*;

pub async fn db_initalize(db: Database) -> Result<Database, mongodb::error::Error> {

    let cols = db.list_collection_names(None).await?;

    if !cols.contains(&"libraries".to_owned()) {
        // check index
        
        let _res = db.collection::<Library>("libraries")
        .create_index(IndexModel::builder()
        .keys(doc!{
            "name": 1i32 })
            .options(Some(IndexOptions::builder()
            .unique(Some(true))
            .build()))
            .build(), None).await?;
        }

    Ok(db)
}

pub async fn db_connection() -> Result<Database, mongodb::error::Error> {

    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(client_options)?;
    
    // TODO switch to connection pool
    let db = client.database("cellstore_rust");
    
    let _test = db.list_collection_names(None).await?;

    let db = db_initalize(db).await?;
    Ok(db)
    
}