use crate::types::*;

use super::*;

use futures::stream::TryStreamExt;

async fn _test_cleanup() {
    todo!()
}


#[tokio::main]
#[test]
async fn can_connect_to_db() {
    let _db = db_connection().await.unwrap();
}

#[tokio::main]
#[test]
async fn library_collection_has_proper_index() {
    let db = db_connection().await.unwrap();
    let index = db.collection::<Library>("libraries").list_indexes(None).await.unwrap();
    let indexes: Vec<mongodb::IndexModel> = index.try_collect().await.unwrap();
    for i in indexes {
        let value = i.keys.get("name");
        match value {
            Some(_) => return,
            None => continue,
        }
    }

    panic!();

}

