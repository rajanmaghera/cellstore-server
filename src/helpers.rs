#[cfg(test)]
mod helpers_test;

use mongodb::{Collection, Database, bson::{self, Bson, doc}};

use crate::types::*;
use crate::error::Error as ApiError;

pub async fn _add_data_type() -> () {
    // is this a do nothing function?
    todo!()
}

pub async fn _remove_data_type() -> () {
    // delete all the versions of the data type here 
    todo!()
}

pub async fn create_library_collection(db: &Database, lib: &Library) -> Result<(), mongodb::error::Error> {
    db.create_collection("library.".to_owned() + &lib.name, None).await
}

pub async fn delete_library_collection(db: &Database, lib: &DeleteLibraryData) -> Result<(), mongodb::error::Error> {
    let col = db.collection::<Bson>(&("library.".to_owned() + &lib.name));
    col.drop(None).await
}

pub async fn get_library_definition_from_string(db: &Database, library_name: &str) -> Result<Library, ApiError> {
    db.collection::<Library>("libraries").find_one(doc! {
        "name": library_name
    }, None).await?.ok_or(ApiError::Implement)

}

pub fn get_library_collection(db: &Database, library: &Library) -> Collection<Bson> {
    db.collection::<Bson>(&("library.".to_owned() + &library.name))
}

pub async fn sanitize_library_item(lib: &Library, item: &Bson) -> Result<Bson, ApiError> {

    // TODO better error message
    
    let item = item.as_document().ok_or(ApiError::Debug("converting to document".to_string()))?;
    let mut doc = doc! { };

    // TODO verify unique names for each library field
    
    // sanitize index
    
    let prim_name = &lib.index.name;
    let prim_type = &lib.index.prim_type;

    match prim_type {
        IndexType::Str => {
            let new_str = item.get_str(prim_name)?;
            doc.insert("index", new_str);
        },
        IndexType::Int =>{
            let new_i32 = item.get_i32(prim_name)?;
            doc.insert("index", new_i32);
        }
    };
    
    // TODO sanitize data types

    let data_doc = doc! {};
    doc.insert("data", &data_doc);


    // TODO implement data types

    // add empty field for results
    // do we need to make this now? can we make it later? It'll save some time now
    // sanitize basic fields

    let mut field_doc = doc! {};
    
    for field in &lib.fields {

        match &field.prim_type {
            PrimType::Str => {
                let new_str = item.get_str(&field.name)?;
                field_doc.insert(&field.name, new_str);
                
            },
            PrimType::Int => {
                let new_i32 = item.get_i32(&field.name)?;
                field_doc.insert(&field.name, new_i32);
            },
            PrimType::Float => {
                let new_f64 = item.get_f64(&field.name)?;
                field_doc.insert(&field.name, new_f64);
            },
            PrimType::Option { values, .. } => {
                // confirm option exists
                let new_i32 = item.get_i32(&field.name)?;
                let i_values = values.iter().map(|x| x.map_to).collect::<Vec<i32>>();
                i_values.contains(&new_i32).then(||()).ok_or(ApiError::Implement)?;
                field_doc.insert(&field.name, new_i32);
            },
        }
    }
    doc.insert("fields", &field_doc);
    
    // return item
    let new_item = bson::from_document(doc)?;


    // TODO return unused fields?
    Ok(new_item)

}

fn _verify_library_unique_fields() {
    todo!()
}