use crate::{db::*};
use crate::error::Error as ApiError;

use super::*;
use super::super::rocket;

use mongodb::bson::{self, Bson};
use rocket::{State, http::{Status}, serde::json::Json};


#[tokio::main]
#[test]
async fn can_create_library() {
    
    let db = db_connection().await.unwrap();
    let rocket = rocket::build().manage(db);
    let state = State::get(&rocket).unwrap();

    let lib = Library {
        name: "test_can_create".to_owned(),
        friendly_name: "Wow! I was made".to_owned(),
        data_types: vec![],
        fields: vec![],
        index: IndexField {
            name: "id".to_owned(),
            friendly_name: "ID".to_owned(),
            prim_type: IndexType::Str,
        },
    };

    let val = Json(lib);

    let res = make_library(state, val).await.unwrap();
    
    assert_eq!(res, Status::Ok);


    // fix borrowing of lib

    let db2 = db_connection().await.unwrap();
    let lib2 = Library {
        name: "test_can_create".to_owned(),
        friendly_name: "Wow! I was made".to_owned(),
        data_types: vec![],
        index: IndexField {
            name: "id".to_owned(),
            friendly_name: "ID".to_owned(),
            prim_type: IndexType::Str,
        },
        fields: vec![],
    };

    let col = db2.collection::<Library>("libraries");
    let result = col.find_one(bson::to_document(&lib2).unwrap(), None).await.unwrap().unwrap();
    
    assert_eq!(result, lib2);

    let del = col.delete_one(bson::to_document(&lib2).unwrap(), None).await.unwrap();

    assert_eq!(del.deleted_count, 1);
    let col = db2.collection::<Bson>(&("library.".to_owned() + &lib2.name));
    col.drop(None).await.unwrap();


}

#[tokio::main]
#[test]
async fn cannot_create_same_titled_library() {
    let db = db_connection().await.unwrap();
    let rocket = rocket::build().manage(db);
    let state = State::get(&rocket).unwrap();

    let lib = Library {
        name: "test_duplicate_name".to_owned(),
        friendly_name: "Wow! I was made".to_owned(),
        data_types: vec![],
        index: IndexField {
            name: "id".to_owned(),
            friendly_name: "ID".to_owned(),
            prim_type: IndexType::Str,
        },
        fields: vec![],
    };

    let val = Json(lib);

    let res = make_library(state, val).await.unwrap();

    assert_eq!(res, Status::Ok);

    let lib = Library {
        name: "test_duplicate_name".to_owned(),
        friendly_name: "Wow! it is a different name".to_owned(),
        data_types: vec![],
        index: IndexField {
            name: "id".to_owned(),
            friendly_name: "ID".to_owned(),
            prim_type: IndexType::Str,
        },
        fields: vec![],
    };
    let val = Json(lib);

    let res = make_library(state, val).await.unwrap_err();

    assert_eq!(res, ApiError::InternalServerError);

    let db = db_connection().await.unwrap();
    let col = db.collection::<Library>("libraries");

    let lib = Library {
        name: "test_duplicate_name".to_owned(),
        friendly_name: "Wow! I was made".to_owned(),
        data_types: vec![],
        index: IndexField {
            name: "id".to_owned(),
            friendly_name: "ID".to_owned(),
            prim_type: IndexType::Str,
        },
        fields: vec![],
    };
    let del = col.delete_one(bson::to_document(&lib).unwrap(), None).await.unwrap();
    
    assert_eq!(del.deleted_count, 1);
    let col = db.collection::<Bson>(&("library.".to_owned() + &lib.name));
    col.drop(None).await.unwrap();

}