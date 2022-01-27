use rocket::{Route, State, fs::TempFile, http::Status, serde::json::Json, form::{Form, Strict}};
use mongodb::{Database, bson::{self, doc}};
use futures::stream::TryStreamExt;

use crate::types::*;
use crate::error::Error as ApiError;
use crate::helpers::*;

#[cfg(test)]
mod routes_test;

pub fn get_routes() -> Vec<Route> {
    routes![
        get_settings,
        set_settings,
        get_libraries_overview,
        get_libraries,
        make_library, 
        delete_library, 
        add_item_to_library, 
        add_items_to_library, 
        file_upload
    ]
}

// TODO create some sort of check for destructive operations


// SETTINGS

#[get("/get_settings")]
async fn get_settings(db: &State<Database>) -> Result<Json<Settings>, ApiError> {
    let collection = db.collection::<Settings>("settings");
    let cursor = collection.find_one(None, None).await;
    match cursor {
        Ok(c) => {
            match c {
                Some(c) => {Ok(Json(c))}
                None => {Err(ApiError::SettingsNotFound)}
            }
        }
        Err(_) => {
            Err(ApiError::SettingsNotFound)
        }
    }

}

#[post("/set_settings", format = "json", data = "<settings>")]
async fn set_settings(db: &State<Database>, settings: Json<Settings>) -> Result<Status, ApiError> {

    let collection = db.collection::<Settings>("settings");
    collection.find_one_and_replace(doc!{}, settings.to_owned(), None).await?;

    Ok(Status::Ok)
}

// LIBRARIES COLLECTION METHODS

#[get("/get_libraries")]
async fn get_libraries(db: &State<Database>) -> Result<Json<Vec<Library>>, ApiError> {

    let collection = db.collection::<Library>("libraries");
    let cursor = collection.find(None, None).await?;

    let v: Vec<Library> = cursor.try_collect().await?;

    Ok(Json(v))
}

#[get("/get_libraries_overview")]
async fn get_libraries_overview(db: &State<Database>) -> Result<Json<LibraryOverview>, ApiError> {
    // include library names,
    let collection = db.collection::<Library>("libraries");
    let cursor = collection.find(None, None).await?;

    let v: Vec<Library> = cursor.try_collect().await?;
    let libs = v.iter().map(|x| x.to_owned().into()).collect();
    let w = LibraryOverview {
         libraries: libs
        };


    Ok(Json(w))
}

#[post("/make_library", format = "json", data = "<library>")]
async fn make_library(db: &State<Database>, library: Json<Library>) -> Result<Status, ApiError> {
    let col = db.collection::<Library>("libraries");

    // TODO verify uniqueness of each item name
    // TODO change to make library element list without accepting default list

    let item = library.into_inner();
    
    col.insert_one(&item, None).await?;

    create_library_collection(db.inner(), &item).await?;

    // TODO add index to column for library
    
    Ok(Status::Ok)

}

// SINGLE LIBRARY METHODS

async fn _get_library() {
    unimplemented!()
}

#[post("/delete_library", format = "json", data = "<lib>")]
async fn delete_library(db: &State<Database>, lib: Json<DeleteLibraryData>) -> Result<Status, ApiError> {
    let col = db.collection::<Library>("libraries");
    let item = lib.into_inner();

    col.delete_one(bson::to_document(&item)?, None).await?;
    delete_library_collection(db.inner(), &item).await?;
    
    Ok(Status::Ok)
}

// LIBRARY COLUMN METHODS

async fn _add_library_column() {
    unimplemented!()
}
async fn _rename_library_column() {
    unimplemented!()
}
async fn _update_library_column() {
    // aka change type
    unimplemented!()
}
async fn _remove_library_column() {
    unimplemented!()
}

// ITEM METHODS

#[post("/add_item_to_library", format = "json", data = "<item>")]
async fn add_item_to_library(db: &State<Database>, item: Json<AddItemData>) -> Result<Status, ApiError> {

    let input = item.into_inner();

    let library = get_library_definition_from_string(db, &input.name).await?;
    let new_item = sanitize_library_item(&library, &input.data).await?;
    let col = get_library_collection(db, &library);
    col.insert_one(new_item, None).await?;
    Ok(Status::Ok)
}

#[post("/add_items_to_library", format = "json", data = "<items>")]
async fn add_items_to_library(db: &State<Database>, items: Json<AddItemsData>) -> Result<Status, ApiError> {

    let input = items.into_inner();

    let library = get_library_definition_from_string(db, &input.name).await?;
    let col = get_library_collection(db, &library);
    
    for item in &input.data {
        // TODO concurrency
        let new_item = sanitize_library_item(&library, &item).await?;
        col.insert_one(new_item, None).await?;
    }
    
    Ok(Status::Ok)
}

// async fn _get_items_in_library(page: i32, limit: i32, sort: Bson) {
//     unimplemented!()
// }

// #[post("/get_item", format = "json", data = "<query>")]
// async fn get_item(db: &State<Database>, query: Json<QueryCell>) -> Result<CellItem, ApiError> {
//     unimplemented!()
// }



// TODO add file upload
#[derive(FromForm)]
struct FileUploadType<'a> {
    file: TempFile<'a>
}

#[post("/file_upload",  format="multipart", data = "<file>")]
async fn file_upload(mut file: Form<Strict<FileUploadType<'_>>>) -> Result<String, ApiError> {
   
    // FIXME nothing worksssss
    // let name =  file.raw_name().ok_or(ApiError::Implement)?.as_str().ok_or(ApiError::Implement)?;
    let filename = format!("uploads/{}", "hey");
    file.file.persist_to(&filename).await?;

   Ok(filename)

}