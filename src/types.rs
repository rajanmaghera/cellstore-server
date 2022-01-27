
use mongodb::bson::Bson;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum DataType {
    Images,
    HekaRaw,
    Processed,
    General
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct DataTypeStore {
    pub data_type: DataType,
    pub data: Bson
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Image {
    // placeholder
    pub filename: String
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum IndexType {
    Str,
    Int
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum PrimType {
    Str,
    Int,
    Float,
    Option {
        values: Vec<OptionValue>,
        tag: bool
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct OptionValue {
    pub name: String,
    pub map_to: i32
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Library {
    pub name: String,
    pub friendly_name: String,
    pub data_types: Vec<DataType>,
    pub fields: Vec<LibraryField>,
    pub index: IndexField
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct LibraryPreview {
    pub name: String,
    pub friendly_name: String,
}


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct LibraryOverview {
    pub libraries: Vec<LibraryPreview>
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Settings {
    pub organization_name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct LibraryField {
    pub name: String,
    pub friendly_name: String,
    pub prim_type: PrimType
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct IndexField {
    pub name: String,
    pub friendly_name: String,
    pub prim_type: IndexType
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AddItemData {
    pub name: String,
    pub data: Bson
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AddItemsData {
    pub name: String,
    pub data: Vec<Bson>
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct DeleteLibraryData {
    pub name: String,
    // add a verification string
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct QueryCell {
    pub library_name: String,
    pub cell_index: String,
}


#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct CellItem {
    pub library_name: String,
    pub data: Bson,
    pub extention: Option<Vec<DataType>>
}


impl From<Library> for LibraryPreview {
    fn from(l: Library) -> Self {
        Self {
            name: l.name,
            friendly_name: l.friendly_name,
        }
    }
}